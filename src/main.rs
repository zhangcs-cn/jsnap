mod io;
mod parser;
mod cli;
mod model;

use std::fs;
use cli::cli::get_args;
use std::path::{Path, PathBuf};
use std::process::exit;
use parser::parser::Parser;

fn main() {
    // 参数
    let (file_path, data_dir) = get_args();

    let file_path = Path::new(&file_path);
    if !file_path.exists() {
        // 文件不存在
        eprintln!("文件不存在: {}", file_path.display());
        exit(1)
    }
    if !file_path.is_file() {
        // 不支持使用文件目录
        eprintln!("请选择需要分析的文件: {}", file_path.display());
        exit(1)
    }

    // 文件名
    let file_stem = file_path.file_stem().unwrap();

    // 工作目录 = {数据目录}/{文件名目录}
    let mut work_path = PathBuf::new();
    work_path.push(data_dir);
    work_path.push(file_stem);
    let result = fs::create_dir_all(work_path.clone());
    if result.is_err() {
        eprintln!("无法初始化工作目录: {}", work_path.display().to_string());
        exit(2)
    }

    let parser = Parser::new(file_path);
    let snapshot = parser.parser().unwrap();
    println!("version: {}", snapshot.get_version());
    println!("oop size: {}", snapshot.get_id_size());
    println!("timestamp: {}", snapshot.get_timestamp());
}


