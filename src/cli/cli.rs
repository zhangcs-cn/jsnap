use clap::builder::Str;
use clap::{Arg, ArgAction, Command, Error};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

const APP_NAME: &str = "jsnap";
pub const FILE_ARG_NAME: &str = "file";
pub const DATA_ARG_NAME: &str = "data";

pub fn get_args() -> (String, String) {
    let matches = cli().get_matches();

    // 文件
    let file = matches.get_one::<String>(FILE_ARG_NAME);
    let file = file.unwrap().to_string();

    // 数据目录
    let data_dir = matches.get_one::<String>(DATA_ARG_NAME);
    let data_dir = if data_dir.is_none() {
        let dir = dirs::home_dir().unwrap_or_else(|| dirs::data_local_dir().unwrap());
        format!("{}{}.jsnap", dir.display(), MAIN_SEPARATOR)
    } else {
        format!("{}", data_dir.unwrap())
    };
    return (file, data_dir);
}

fn cli() -> Command {
    Command::new(APP_NAME)
        .about("Java堆转储快照文件分析工具")
        .author("zcs")
        .version("0.1.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new(DATA_ARG_NAME)
                .short('d')
                .long("data")
                .action(ArgAction::Set)
                .help("数据文件目录"),
        )
        .arg(
            Arg::new(FILE_ARG_NAME)
                .required(true)
                .help("需要分析的快照文件")
                .action(ArgAction::Set)
                .num_args(1),
        )
}