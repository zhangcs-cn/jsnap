use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, ErrorKind, SeekFrom};
use std::path::PathBuf;
use std::io::Error;
use crate::parser::dump::get_heap_dump;
use crate::parser::reader;
use crate::parser::reader::{AllocSites, ControlSettings, CpuSamples, Frame, HeapSummary, Class, Reader, Thread, Trace, Utf8};
use crate::io::channel::{Result, Byte, Int, Long};
use derive_getters::Getters;

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

#[derive(Clone, Debug, Getters)]
pub struct Hprof {
    file_name: String,
    id_size: Long,
    version: String,
    timestamp: Long,
}

/// 解析堆转储快照文件
pub fn read(file_path: &PathBuf, _: &PathBuf) -> Result<Hprof> {
    let file_name = file_path.to_str().unwrap().to_string();
    let mut reader = Reader::new(&file_path)?;

    // 版本
    let mut version = String::new();
    for index in 1..20 {
        if index > 20 {
            break;
        }
        let byte = reader.read_char();
        if byte == '\0' {
            break;
        }
        version.push(byte);
    }

    if version != HPROF_HEADER_101 && version != HPROF_HEADER_102 {
        eprintln!("不支持的版本: {}", version);
        return Err(Error::new(ErrorKind::Unsupported, format!("不支持的版本: {}", version)));
    };

    // oop size
    let id_size = reader.get_id_size();

    // 时间戳（毫秒）
    let timestamp = reader.get_timestamp();

    let mut symbols: HashMap<u64, String> = HashMap::new();

    loop {
        let header = reader.get_header();
        if header.is_err() {
            let err = header.unwrap_err();
            if err.kind() != ErrorKind::UnexpectedEof {
                // 非读取到文件末尾
                eprintln!("解析异常: {}", err.to_string());
                return Err(err);
            }
            break;
        }
        let (tag, _, length) = header.unwrap();
        match tag {
            HPROF_UTF8 => {
                // a UTF8-encoded name
                let utf8 = reader.read::<Utf8>(length);
                symbols.insert(*utf8.symbol_id(), utf8.name().to_string());
            }
            HPROF_LOAD_CLASS => {
                // a newly loaded class
                let class = reader.read::<Class>(length);
                let class_name = get_name_from_id(class.name_id(), &symbols);
                println!("{}", class_name)
            }
            HPROF_UNLOAD_CLASS => {
                // an unloading class
                let ser_num = reader.read_int();
                println!("unload {}", ser_num)
            }
            HPROF_FRAME => {
                // a Java stack frame
                let frame = reader.read::<Frame>(length);
                // 方法名
                let method_name = symbols.get(frame.method_name());
                // 方法签名
                let method_sig = symbols.get(frame.method_sig());
                // 源文件
                let src_file = symbols.get(frame.src_file());
                println!("frame -> {} {} {}", method_name.unwrap(), method_sig.unwrap(), src_file.unwrap());
            }
            HPROF_TRACE => {
                // a Java stack trace
                let trace = reader.read::<Trace>(length);
                // println!("{}", trace);
            }
            HPROF_ALLOC_SITES => {
                // a set of heap allocation sites, obtained after GC
                let sites = reader.read::<AllocSites>(length);

            }
            HPROF_HEAP_SUMMARY => {
                // heap summary
                let summary = reader.read::<HeapSummary>(length);
                println!("summary: {}", summary.live())
            }
            HPROF_START_THREAD => {
                // a newly started thread.
                let thread = reader.read::<Thread>(length);
                println!("0x{}", format!("{:x}", thread.id()));
            }
            HPROF_END_THREAD => {
                // a terminating thread.
                let thread_serial_num = reader.read_int();
            }
            HPROF_CPU_SAMPLES => {
                // a set of sample traces of running threads
                let samples = reader.read::<CpuSamples>(length);
                println!("cpu samples: {}", samples.num())
            }
            HPROF_CONTROL_SETTINGS => {
                // the settings of on/off switches
                let settings = reader.read::<ControlSettings>(length);
                println!("settings: {}", settings.flags())
            }
            HPROF_HEAP_DUMP => {
                // denote a heap dump
                // let _ = get_heap_dump(&mut reader, length);
                reader.skip(length);
            }
            HPROF_HEAP_DUMP_SEGMENT => {
                // denote a heap dump segment
                reader.skip(length);
            }
            HPROF_HEAP_DUMP_END => {
                //  denotes the end of a heap dump
                reader.skip(length);
            }
            _ => {
                reader.skip(length);
            }
        }
    }

    Ok(Hprof { file_name, id_size: id_size as u64, version, timestamp })
}

fn get_name_from_id(id: &Long, symbols: &HashMap<Long, String>) -> String {
    if *id == 0 {
        return "".to_string();
    }

    let name = symbols.get(id);
    if name.is_none() {
        let mut name = String::from("unresolved name ");
        name.push_str(&*id.to_string());
        return name;
    }

    let name = name.unwrap();
    name.replace("/", ".")
}