use std::collections::HashMap;
use std::io;
use std::io::{Error, ErrorKind};
use std::result::Result;
use std::path::{Path, PathBuf};
use crate::cli::bar::Bar;
use crate::db::class::{LoadClassDao};
use crate::io::error::NotSupport;
use super::super::io::wrapper::ChannelWrapper;
use super::super::io::error::EndOfFile;
use super::super::model::snapshot::Snapshot;
use super::super::db::symbol::SymbolDao;
use super::super::cli::bar;

const HPROF_HEADER_101: &str = "JAVA PROFILE 1.0.1";
const HPROF_HEADER_102: &str = "JAVA PROFILE 1.0.2";

const HPROF_UTF8: u8 = 0x01;
const HPROF_LOAD_CLASS: u8 = 0x02;
const HPROF_UNLOAD_CLASS: u8 = 0x03;
const HPROF_FRAME: u8 = 0x04;
const HPROF_TRACE: u8 = 0x05;
const HPROF_ALLOC_SITES: u8 = 0x06;
const HPROF_HEAP_SUMMARY: u8 = 0x07;
const HPROF_START_THREAD: u8 = 0x0A;
const HPROF_END_THREAD: u8 = 0x0B;
const HPROF_HEAP_DUMP: u8 = 0x0C;
const HPROF_CPU_SAMPLES: u8 = 0x0D;
const HPROF_CONTROL_SETTINGS: u8 = 0x0E;

const HPROF_HEAP_DUMP_SEGMENT: u8 = 0x1C;
const HPROF_HEAP_DUMP_END: u8 = 0x2C;

pub struct Parser {
    file_path: PathBuf,
    work_path: PathBuf,
}

impl Parser {
    pub(crate) fn new(file_path: &Path, work_path: &Path) -> Parser {
        Parser {
            file_path: file_path.to_path_buf(),
            work_path: work_path.to_path_buf(),
        }
    }

    pub fn parser(&self) -> Result<Snapshot, Error> {
        let mut channel = ChannelWrapper::wrapper(&self.file_path)?;
        let total_size = channel.size();
        let bar = Bar::new(String::from("Reading file"), total_size);

        // 版本
        let mut version = String::new();
        for index in 1..20 {
            if index > 20 {
                break;
            }
            let byte = channel.read_char()?;
            if byte == '\0' {
                break;
            }
            version.push(byte);
        }

        if version != HPROF_HEADER_101 && version != HPROF_HEADER_102 {
            eprintln!("不支持的版本: {}", version);
            return Err(Error::from(NotSupport));
        };
        bar.inc(version.len() as u64);

        // oop size
        let id_size = channel.read_id_size()?;
        bar.inc(4);

        // 时间戳（毫秒）
        let timestamp = channel.read_long()?;
        bar.inc(8);

        // 快照实体
        let snapshot = Snapshot::new(id_size, version, timestamp);

        let mut symbols: HashMap<u64, String> = HashMap::new();

        let mut classes: HashMap<u32, (u32, u64, u64, String, u32)> = HashMap::new();

        loop {
            let header = channel.read_header();
            if header.is_err() {
                break;
            }
            let (tag, offset, length) = header?;
            match tag {
                HPROF_UTF8 => {
                    let (symbol_id, symbol_name) = channel.read_utf8(length)?;
                    symbols.insert(symbol_id, symbol_name);

                    bar.inc((id_size + (length - id_size)) as u64);
                }
                HPROF_LOAD_CLASS => {
                    let (serial_num, class_id, _, class_name_id) = channel.read_load_class()?;
                    let class_name = if class_name_id == 0 {
                        "".to_string()
                    } else {
                        let mut name = String::from("unresolved name ");
                        name.push_str(&*class_name_id.to_string());
                        let name = symbols.get(&class_name_id).unwrap_or(&name);
                        name.replace("/", ".")
                    };
                    classes.insert(serial_num, (serial_num, class_id, class_name_id, class_name, 1 as u32));

                    bar.inc(8 + (2 * id_size) as u64);
                }
                HPROF_UNLOAD_CLASS => {
                    let class_ser_num = channel.read_unload_class()?;
                    let (_, _, _, _, class_status) = classes.get_mut(&class_ser_num).unwrap();
                    *class_status = 0;

                    bar.inc(4);
                }
                HPROF_FRAME => {
                    let (frame_id, method_name, method_sig, src_file, class_ser_num, line_nr) = channel.read_frame()?;
                    bar.inc(8 + (4 * id_size) as u64);
                }
                HPROF_TRACE => {
                    // 堆栈
                    let (stack_trace_nr, thread_nr, frame_ids) = channel.read_hprof_trace()?;
                    bar.inc(12 + (frame_ids.len() * (id_size as usize)) as u64);
                }
                HPROF_START_THREAD => {
                    // 线程
                    let (thread_serial_num,
                        thread_obj_id,
                        trace_serial_num,
                        t_name_index,
                        g_name_index,
                        p_name_index,
                        id_size) = channel.read_start_thread()?;
                    println!("thread: {}", thread_serial_num);
                    bar.inc(8 + (4 * id_size) as u64);
                }
                HPROF_END_THREAD => {
                    // 已结束线程
                    let thread_serial_num = channel.read_int()?;
                    bar.inc(4);
                    println!("end thread: {}", thread_serial_num);
                }
                HPROF_HEAP_SUMMARY => {
                    // 堆摘要
                    let (live, live_inst, allocate, allocate_inst) = channel.read_heap_summary()?;
                    bar.inc(24);
                    println!("summary: live={}, live_inst={}, allocate={}, allocate_inst={}", live, live_inst, allocate, allocate_inst)
                }
                _ => {
                    channel.skip(length);
                    bar.inc(length as u64);
                }
            }
        }

        // 符号
        let symbol_dao = SymbolDao::new(&self.work_path);
        let mut symbol_dao = match symbol_dao {
            Ok(dao) => dao,
            Err(error) => return Err(Error::new(ErrorKind::Other, error.to_string()))
        };
        symbol_dao.add_all(symbols);

        // 类
        let class_dao = LoadClassDao::new(&self.work_path);
        let mut class_dao = match class_dao {
            Ok(dao) => dao,
            Err(error) => return Err(Error::new(ErrorKind::Other, error.to_string()))
        };
        let values = classes.values().cloned().collect();
        class_dao.add_all_class(values);

        bar.finish_with_message("Finished");

        Ok(snapshot)
    }
}


