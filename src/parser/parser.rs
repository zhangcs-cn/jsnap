use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::result::Result;
use std::path::{Path, PathBuf};
use super::super::io::wrapper::ChannelWrapper;
use super::super::io::error::EndOfFile;
use super::super::model::snapshot::Snapshot;
use super::super::db::symbol::SymbolDao;

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
            panic!("不支持的版本: {}", version)
        };

        // oop size
        let id_size = channel.read_id_size()?;

        // 时间戳（毫秒）
        let timestamp = channel.read_long()?;

        // 快照实体
        let snapshot = Snapshot::new(id_size, version, timestamp);

        let mut symbols: HashMap<u64, String> = HashMap::new();
        let mut classes: Vec<(u32, u64, u64)> = Vec::new();
        let mut un_load_classes: Vec<(u32)> = Vec::new();

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
                }
                HPROF_LOAD_CLASS => {
                    let (serial_num, class_id, name_id) = channel.read_load_class()?;
                    classes.push((serial_num, class_id, name_id));
                }
                HPROF_UNLOAD_CLASS => {
                    let class_ser_num = channel.read_unload_class()?;
                    un_load_classes.push(class_ser_num)
                }
                HPROF_FRAME => {
                    let (frame_id, method_name, method_sig, src_file, class_ser_num, line_nr) = channel.read_frame()?;
                }
                HPROF_TRACE => {
                    // 堆栈
                    let (stack_trace_nr, thread_nr, frame_ids) = channel.read_hprof_trace()?;
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
                }
                HPROF_END_THREAD => {
                    // 已结束线程
                    let thread_serial_num = channel.read_int()?;
                }
                HPROF_HEAP_SUMMARY => {
                    // 堆摘要
                    let (live, live_inst, allocate, allocate_inst) = channel.read_heap_summary()?;
                }
                _ => {
                    channel.skip(length);
                }
            }
        }

        let symbol_dao = SymbolDao::new(&self.work_path);
        let mut symbol_dao = match symbol_dao {
            Ok(dao) => dao,
            Err(error) => return Err(Error::new(io::ErrorKind::Other, error.to_string()))
        };
        symbol_dao.add_all(symbols);

        Ok(snapshot)
    }
}


