use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufReader, Cursor, ErrorKind, SeekFrom};
use std::path::PathBuf;
use std::io::Result;
use std::io::Error;
use crate::parser::channel::{Channel, Long};

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


pub fn parse(file_path: PathBuf, work_dir: PathBuf) -> Result<Hprof> {
    let file_name = file_path.to_str().unwrap().to_string();
    let mut channel = Channel::new(file_path)?;

    // 版本
    let mut version = get_version(&mut channel)?;

    // oop size
    let id_size = channel.read_int()?;

    // 时间戳（毫秒）
    let timestamp = channel.read_long()?;

    Ok(Hprof { file_name, id_size: id_size as u64, version, timestamp })
}

/// 获取版本号
fn get_version(channel: &mut Channel) -> Result<String> {
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
        return Err(Error::new(ErrorKind::Unsupported, format!("不支持的版本: {}", version)));
    };
    Ok(version)
}


