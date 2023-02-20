mod io;

use std::ffi::OsStr;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};
use clap::{Arg, ArgAction, Command};
use dirs;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};
use clap::builder::Str;
use dirs::data_dir;
use io::channel::Channel;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

pub const APP_NAME: &str = "jsnap";
pub const FILE_ARG_NAME: &str = "file";
pub const DATA_ARG_NAME: &str = "data";

fn cli() -> Command {
    Command::new(APP_NAME)
        .about("Java Snapshot Analysis Tool")
        .author("zcs")
        .version("0.1.0")
        .arg_required_else_help(true)
        .arg(Arg::new(DATA_ARG_NAME)
            .short('d')
            .long("data")
            .action(ArgAction::Set)
            .help("data file directory")
        )
        .arg(Arg::new(FILE_ARG_NAME)
            .required(true)
            .help("a snapshots file")
            .action(ArgAction::Set)
            .num_args(1))
}

fn main() -> std::io::Result<()> {
    let matches = cli().get_matches();

    let file = matches.get_one::<String>(FILE_ARG_NAME).unwrap();
    println!("file -> {}", file);

    let file_path = Path::new(file);
    if !file_path.exists() {
        eprintln!("文件不存在 '{}'", file_path.to_str().unwrap());
    }
    if file_path.is_dir() {
        eprintln!("不支持指定文件夹 '{}'", file_path.to_str().unwrap());
    }

    let file_stem = file_path.file_stem().unwrap();
    println!("{:?}", file_stem);

    let data_dir = matches.get_one::<String>(DATA_ARG_NAME);
    let data_dir_path: String = if data_dir.is_none() {
        let dir = dirs::home_dir().unwrap_or_else(|| {
            dirs::data_local_dir().unwrap()
        });
        format!("{}{}.jsnap", dir.display(), MAIN_SEPARATOR)
    } else {
        format!("{}", data_dir.unwrap())
    };
    println!("data -> {}", data_dir_path);

    let mut work_path = PathBuf::new();
    work_path.push(data_dir_path);
    work_path.push(file_stem);
    println!("{}", work_path.display());
    fs::create_dir_all(work_path.clone()).expect(format!("无法写数据{}", work_path.display()).as_str());

    let channel = Channel::new(file_path);
    if channel.is_err() {
        panic!("{}", channel.err().unwrap())
    }

    let mut channel = channel.unwrap();
    let version = channel.read_str(18)?;
    println!("version: {}", version);

    channel.skip(1);

    let id_size = channel.read_u32()?;
    println!("oop size: {}", id_size);

    let timestamp = channel.read_u64()?;
    println!("timestamp: {}", timestamp);

    let mut constant_pool = HashMap::new();
    let mut class_2_name = HashMap::new();
    let mut class_ser_num_2_id = HashMap::new();

    loop {
        // let tag: Vec<u8> = vec![Default::default(); 1];
        let mut tag: [u8; 1] = [0; 1];
        let count = channel.reads(&mut tag);
        if count == 0 {
            break;
        }
        let tag = tag.get(0).unwrap().clone();

        let offset = channel.read_u32()?;
        let length = channel.read_u32()?;
        match tag {
            HPROF_UTF8 => {
                // 常量池
                let mut length = length - id_size;
                let symbol_id = channel.read_id()?;
                let name = channel.read_str(length as usize)?;
                constant_pool.insert(symbol_id, name);
            }
            HPROF_LOAD_CLASS => {
                // 加载类
                let serial_num = channel.read_u32()?;
                let class_id = channel.read_u32()?;
                channel.skip(4);
                let name_id = channel.read_id()?;

                let class_name = constant_pool.get(&name_id);
                let class_name = match class_name {
                    Some(name) => name,
                    None => ""
                };
                let class_name = class_name.replace("/", ".");
                println!("{}", class_name);
                class_2_name.insert(class_id, class_name);
                class_ser_num_2_id.insert(serial_num, class_id);
            }
            HPROF_UNLOAD_CLASS => {
                let class_ser_num = channel.read_u32()?;
                let class_id = class_ser_num_2_id.get(&class_ser_num);
                if class_id.is_some() {
                    class_2_name.remove(class_id.unwrap());
                }
            }
            HPROF_FRAME => {
                let frame_id = channel.read_id();
                let method_name = channel.read_id();
                let method_sig = channel.read_id();
                let src_file = channel.read_id();
                let class_ser_num = channel.read_u32();
                let line_nr = channel.read_u32();

            }
            _ => {
                channel.skip(length as i64)
            }
        }
    }

    test(100);

    Ok(())
}

fn test(len: u64) {
    let pd = ProgressBar::new(len);
    pd.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>7} {len:7} [{elapsed_precise}]")
            .unwrap()
            .progress_chars("#>-")
    );

    for _ in 0..100 {
        pd.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn read_utf8(channel: &mut Channel, len: i64) {
    // 常量池
    let mut length = len - id_size;
    let symbol_id = channel.read_id()?;
    let name = channel.read_str(length as usize)?;
    constant_pool.insert(symbol_id, name);
}

fn read_load_class() {

}

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