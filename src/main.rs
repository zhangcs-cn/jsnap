mod args;
mod parser;
mod errors;
mod io;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use crate::args::Args;
use crate::parser::hprof;

fn main() {
    // 启动参数
    let args = args::get_args();

    // 初始化数据目录
    let (file_path, work_path) = init_dir(args);

    // 解析
    let hprof = hprof::parse(&file_path, &work_path);
    let _ = match hprof {
        Ok(hprof) => hprof,
        Err(_) => {
            eprintln!("解析失败");
            exit(exitcode::DATAERR)
        }
    };

    exit(exitcode::OK)
}

/// 初始化工作目录
fn init_dir(args: Args) -> (PathBuf, PathBuf) {
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

    let file_name = match file_path.file_name() {
        None => {
            eprintln!("无法识别文件名: {}", file_path.display());
            exit(exitcode::OSFILE)
        }
        Some(name) => name
    };
    let file_name = file_name.to_string_lossy().to_string();

    // 工作目录 = {数据目录}/{文件名目录}
    let mut work_path = PathBuf::new();
    work_path.push(args.get_data_dir());
    work_path.push(file_name);

    if work_path.exists() && work_path.is_dir() {
        println!("工作目录: {}", get_path_real_name(&mut work_path));
        return (file_path.to_path_buf(), work_path);
    }

    let result = fs::create_dir_all(work_path.clone());
    if result.is_err() {
        eprintln!("无法初始化工作目录: {}", get_path_real_name(&mut work_path));
        exit(exitcode::CANTCREAT)
    }
    println!("工作目录: {}", work_path.canonicalize().unwrap().display().to_string());

    (file_path.to_path_buf(), work_path)
}

fn get_path_real_name(path: &mut PathBuf) -> String {
    path.canonicalize().unwrap().display().to_string()
}

#[test]
fn test_main() {

}