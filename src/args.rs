use clap::{Command, Arg, ArgAction};
use std::path::{MAIN_SEPARATOR};

const APP_NAME: &str = "JSnap";
const APP_DESCRIBE: &str = "a java snapshot analysis tool";
const APP_AUTHOR: &str = "zhangcs";
const APP_VERSION: &str = "0.1.0";

/// 获取应用启动参数
/// ```
/// # jsnap [-d <data_dir>] [-r] <file>
/// let args = args::get_args();
/// ```
pub fn get_args() -> Args {
    let file_arg_name = "file";
    let data_arg_name = "data";
    let force_arg_name = "force";
    let data_arg = Arg::new(data_arg_name)
        .short('d')
        .long("data")
        .action(ArgAction::Set)
        .help("数据文件存储路径");
    let force_arg = Arg::new(force_arg_name)
        .short('r')
        .long("force")
        .action(ArgAction::SetTrue)
        .help("强制重新分析文件");
    let file_arg = Arg::new(file_arg_name)
        .required(true)
        .help("快照文件")
        .action(ArgAction::Set)
        .num_args(1);

    let matches = Command::new(APP_NAME)
        .about(APP_DESCRIBE)
        .author(APP_AUTHOR)
        .version(APP_VERSION)
        .arg_required_else_help(true)
        .arg(data_arg)
        .arg(force_arg)
        .arg(file_arg)
        .get_matches();

    // 快照文件
    let file = matches.get_one::<String>(file_arg_name);
    let file = file.unwrap().to_string();

    // 数据目录
    let data_dir = matches.get_one::<String>(data_arg_name);
    let data_dir = match data_dir {
        Some(data_dir) => data_dir.to_string(),
        None => format!(".{}.jsnap", MAIN_SEPARATOR)
    };

    // 重新分析
    let force = matches.get_flag(force_arg_name);

    // 返回
    Args::new(file, data_dir, force)
}

/// 启动命令参数
pub struct Args {
    file: String,
    data_dir: String,
    force: bool,
}

impl Args {
    fn new(file: String, data_dir: String, force: bool) -> Args {
        Args { file, data_dir, force }
    }
    pub fn get_file(&self) -> &String {
        &self.file
    }
    pub fn get_data_dir(&self) -> &String {
        &self.data_dir
    }
    pub fn is_force(&self) -> &bool {
        &self.force
    }
}