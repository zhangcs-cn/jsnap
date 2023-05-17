use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufReader, Cursor, ErrorKind, SeekFrom};
use std::path::PathBuf;
use std::io::Error;
use crate::parser::dump::get_heap_dump;
use crate::parser::reader;
use crate::parser::reader::{Frame, HeapSummary, LoadedClass, Reader, StartThread, Trace, UnLoadClass, Utf8};
use crate::io::channel::{Result, Byte, Int, Long};

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

pub struct Hprof {
    file_name: String,
    id_size: Long,
    version: String,
    timestamp: Long,
}

impl Hprof {
    pub fn get_version(&self) -> &String {
        &self.version
    }
    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }
}

/// 解析堆转储快照文件
pub fn parse(file_path: &PathBuf, work_dir: &PathBuf) -> Result<Hprof> {
    let file_name = file_path.to_str().unwrap().to_string();
    let mut reader = Reader::new(&file_path)?;

    // 版本
    let mut version = String::new();
    for index in 1..20 {
        if index > 20 {
            break;
        }
        let byte = reader.read_char()?;
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
    let id_size = reader.get_id_size()?;

    // 时间戳（毫秒）
    let timestamp = reader.get_timestamp()?;

    loop {
        let header = reader.get_header();
        if header.is_err() {
            break;
        }
        let (tag, offset, length) = header?;
        match tag {
            HPROF_UTF8 => {
                // a UTF8-encoded name
                let utf8 = reader.read::<Utf8>(length);
                println!("{} = {}", utf8.symbol_id(), utf8.name())
            }
            HPROF_LOAD_CLASS => {
                // a newly loaded class
                let loaded_class = reader.read::<LoadedClass>(length);
            }
            HPROF_UNLOAD_CLASS => {
                // an unloading class
                let unload_class = reader.read::<UnLoadClass>(length);
            }
            HPROF_FRAME => {
                // a Java stack frame
                let frame = reader.read::<Frame>(length);
            }
            HPROF_TRACE => {
                // a Java stack trace
                // let (stack_trace_nr, thread_nr, frame_ids) = reader.get_hprof_trace()?;
                let trace = reader.read::<Trace>(length);
                println!("stack_trace_nr={}, thread_nr={}", trace.stack_trace_nr(), trace.thread_nr());
            }
            HPROF_ALLOC_SITES => {
                // a set of heap allocation sites, obtained after GC
                reader.skip(length);
            }
            HPROF_HEAP_SUMMARY => {
                // heap summary
                let summary = reader.read::<HeapSummary>(length);
            }
            HPROF_START_THREAD => {
                // a newly started thread.
                let thread = reader.read::<StartThread>(length);
            }
            HPROF_END_THREAD => {
                // a terminating thread.
                let thread_serial_num = reader.read_int();
            }
            HPROF_HEAP_DUMP => {
                // denote a heap dump
                // reader.skip(length);
                let _ = get_heap_dump(&mut reader, length);
            }
            HPROF_CPU_SAMPLES => {
                // a set of sample traces of running threads
                reader.skip(length);
            }
            HPROF_CONTROL_SETTINGS => {
                // the settings of on/off switches
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

