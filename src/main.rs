use std::path::{Path, PathBuf};
use std::process::exit;
use std::fs;
use std::fs::remove_dir_all;
use crate::args::Args;

mod args;

fn main() {
    // 启动参数
    let args = args::get_args();

    // 初始化数据目录
    let work_path = init_dir(args);


    exit(exitcode::OK)
}

/// 初始化工作目录
fn init_dir(args: Args) -> PathBuf {
    // 快照文件
    let file_path = Path::new(args.get_file());
    if !file_path.exists() {
        // 文件不存在
        eprintln!("文件不存在: {}", args.get_file());
        exit(exitcode::OSFILE)
    }
    if !file_path.is_file() {
        // 未指定具体文件
        eprintln!("不支持使用目录，请指定要分析的文件: {}", args.get_file());
        exit(exitcode::DATAERR)
    }
    println!("快照文件: {}", args.get_file());

    let file_stem = file_path.file_stem();
    if file_stem.is_none() {
        eprintln!("无法识别文件名: {}", file_path.display());
        exit(exitcode::OSFILE)
    }
    let file_stem = file_stem.unwrap();

    // 工作目录 = {数据目录}/{文件名目录}
    let mut work_path = PathBuf::new();
    work_path.push(args.get_data_dir());
    work_path.push(file_stem);
    println!("工作目录: {}", work_path.to_str().unwrap());

    if work_path.exists() && work_path.is_dir() {
        return work_path;
    }

    let result = fs::create_dir_all(work_path.clone());
    if result.is_err() {
        eprintln!("无法初始化工作目录: {}", work_path.display().to_string());
        exit(exitcode::CANTCREAT)
    }

    work_path
}