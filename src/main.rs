mod snap;

use std::ffi::OsStr;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};
use clap::{Arg, ArgAction, Command};
use dirs;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};
use clap::builder::Str;
use dirs::data_dir;
use indicatif::ProgressBar;
use snap::channel::HprofChannel;

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

    let channel = HprofChannel::new(file_path);
    if channel.is_err() {
        panic!("{}", channel.err().unwrap())
    }

    let mut channel = channel.unwrap();
    let version = channel.read_str(18)?;
    println!("version: {}", version);

    channel.skip(1);

    let idSize = channel.read_u32()?;
    println!("oop size: {}", idSize);

    let timestamp = channel.read_u64()?;
    println!("timestamp: {}", timestamp);

    Ok(())
}

fn test() {
    let bar = ProgressBar::new(1000);
    for _ in 0..1000 {
        bar.inc(1);
    }
    bar.finish();
}
