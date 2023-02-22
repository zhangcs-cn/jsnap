use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::Path;
use std::process::id;
use std::sync::{Arc, Mutex};
use clap::Error;
use indicatif::ProgressBar;
use crate::io::channel::Channel;
use super::channel::HprofChannel;
use super::bar::Bar;

static mut ID_SIZE: u32 = 8;

// lazy_static::lazy_static! {
//     static ref CONSTANT_POOL: HashMap<u64, String>  HashMap::new();
// }

pub struct Snapshot {
    version: String,
    id_size: u32,
    timestamp: u64,
    constant_pool: HashMap<u64, String>,
    class_2_name: HashMap<u32, String>,
    class_ser_num_2_id: HashMap<u32, u32>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        return Snapshot {
            version: String::from("unknown"),
            id_size: 8,
            timestamp: 0,
            constant_pool: Default::default(),
            class_2_name: Default::default(),
            class_ser_num_2_id: Default::default(),
        };
    }
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    pub fn get_version(&mut self) -> &str {
        &self.version
    }
    pub fn get_id_size(&mut self) -> u32 {
        self.id_size
    }
    pub fn get_timestamp(&mut self) -> u64 {
        self.timestamp
    }
    pub fn set_constant(&mut self, symbol_id: u64, name: String) {
        self.constant_pool.insert(symbol_id, name);
    }
    pub fn get_constant(&mut self, symbol_id: u64) -> &str {
        let class_name = self.constant_pool.get(&symbol_id);
        match class_name {
            None => "unknown",
            Some(class_name) => class_name
        }
    }
    pub fn set_class_name(&mut self, class_id: u32, class_name: String) {
        self.class_2_name.insert(class_id, class_name);
    }
    pub fn get_class_name(&mut self, class_id: u32) -> &str {
        let class_name = self.class_2_name.get(&class_id);
        match class_name {
            None => "unknown",
            Some(class_name) => class_name
        }
    }
    pub fn set_class_ser_num_2_id(&mut self, serial_num: u32, class_id: u32) {
        self.class_ser_num_2_id.insert(serial_num, class_id);
    }
    pub fn get_class_ser_num_2_id(&mut self, serial_num: u32) -> u32 {
        let class_id = self.class_ser_num_2_id.get(&serial_num);
        match class_id {
            None => 0,
            Some(class_id) => *class_id
        }
    }
    pub fn get_id(&mut self, channel: &mut dyn Channel) -> Result<u64, Error> {
        let id: u64;
        if self.id_size == 4 {
            id = channel.read_u32()? as u64;
        } else {
            id = channel.read_u64()?;
        }
        Ok(id)
    }
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

const HPROF_HEAP_DUMP_SEGMENT: u8 = 0x1C;
const HPROF_HEAP_DUMP_END: u8 = 0x2C;

const HPROF_HEADER_101: &str = "JAVA PROFILE 1.0.1";
const HPROF_HEADER_102: &str = "JAVA PROFILE 1.0.2";

pub fn parse(file: &Path) -> Result<Snapshot, Error> {
    let channel = HprofChannel::new(file);
    if channel.is_err() {
        panic!("{}", channel.err().unwrap())
    }

    let mut channel = channel.unwrap();
    let bar = Bar::new(channel.size());

    let version = read_version(&mut channel);
    bar.inc((version.len() + 1) as u64);

    let id_size = channel.read_u32()?;
    bar.inc(4);

    unsafe {
        ID_SIZE = id_size;
    }
    let timestamp = channel.read_u64()?;
    bar.inc(8);

    let mut snap: Snapshot = Snapshot::new();

    let mut constant_pool: HashMap<u64, String> = HashMap::new();
    let mut class_2_name: HashMap<u32, String> = HashMap::new();
    let mut class_ser_num_2_id: HashMap<u32, u32> = HashMap::new();

    loop {
        let mut tag: [u8; 1] = [0; 1];
        let count = channel.reads(&mut tag);
        if count == 0 {
            break;
        }
        let tag = tag.get(0);
        let tag = *tag.unwrap();
        bar.inc(1);

        let offset = channel.read_u32()?;
        bar.inc(4);
        let length = channel.read_u32()?;
        bar.inc(4);
        match tag {
            HPROF_UTF8 => {
                let (symbol_id, name) = read_utf8(&mut channel, length, &bar);
                constant_pool.insert(symbol_id, name);
            }
            HPROF_LOAD_CLASS => {
                let (serial_num, class_id, name_id) = read_load_class(&mut channel, &bar);
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
                let class_ser_num = channel.read_u32();
                bar.inc(4);
                let class_id = class_ser_num_2_id.get(&class_ser_num.unwrap());
                class_2_name.remove(class_id.unwrap());
            }
            HPROF_FRAME => read_frame(&mut channel, &bar),
            HPROF_TRACE => {
                // 堆栈
                read_hprof_trace(&mut channel, &bar);
            }
            HPROF_START_THREAD => {
                // 线程
                read_start_thread(&mut channel, &bar);
            }
            HPROF_END_THREAD => {
                // 已结束线程
                let threadSerialNum = channel.read_u32();
                bar.inc(4);
            }
            HPROF_HEAP_SUMMARY => {
                // 堆摘要
                read_heap_summary(&mut channel, &bar);
            }
            HPROF_CONTROL_SETTINGS => {
                // 开关配置
                // 0x00000001: alloc traces on/off
                // 0x00000002: cpu sampling on/off
                let settings = channel.read_u32();
                // stack trace depth
                let maxTraceDepth = channel.read_u16();
            }
            HPROF_ALLOC_SITES => {
                // a set of heap allocation sites, obtained after GC
                // 0x0001: incremental vs. complete
                // 0x0002: sorted by allocation vs. live
                // 0x0004: whether to force a GC
                let flags = channel.read_u16();
                // cutoff ratio
                let cutoff = channel.read_u32();
                // total live bytes
                let nblive = channel.read_u32();
                // total live instances
                let nilive = channel.read_u32();
                // total bytes allocated
                let tbytes = channel.read_u64();
                // total instances allocated
                let tinsts = channel.read_u64();
                // number of sites that follow
                let num_elements = channel.read_u32().unwrap();
                for i in 0..num_elements {
                    let ty = channel.read_u8();
                    let classSerialNum = channel.read_u32();
                    let traceSerialNum = channel.read_u32();
                    let nblive = channel.read_u32();
                    let nilive = channel.read_u32();
                    let tbytes2 = channel.read_u32();
                    let tinsts2 = channel.read_u32();
                }
            }
            HPROF_CPU_SAMPLES => {
                channel.skip(length as i64);
                bar.inc(length as u64);
            }
            HPROF_HEAP_DUMP_END => {
                channel.skip(length as i64);
                bar.inc(length as u64);
            }
            _ => {
                channel.skip(length as i64);
                bar.inc(length as u64);
            }
        }
    }

    bar.finish_and_clear();
    println!("读取文件完成");
    println!("version: {}", version);
    println!("oop size: {}", id_size);
    println!("timestamp: {}", timestamp);

    return Ok(snap);
}

fn read_utf8(channel: &mut HprofChannel, len: u32, bar: &ProgressBar) -> (u64, String) {
    // 常量池
    let size = unsafe {
        ID_SIZE
    };
    let mut length = len - size;
    let symbol_id = read_id(channel);
    let name = channel.read_str(length as usize);
    bar.inc(size as u64);
    bar.inc(length as u64);
    return (symbol_id, name.unwrap());
}

fn read_load_class(channel: &mut HprofChannel, bar: &ProgressBar) -> (u32, u32, u64) {
    // 加载类
    let serial_num = channel.read_u32().unwrap();
    bar.inc(4);
    let class_id = channel.read_u32().unwrap();
    bar.inc(4);
    channel.skip(4);
    bar.inc(4);
    let name_id = read_id(channel);
    bar.inc(unsafe { ID_SIZE } as u64);
    return (serial_num, class_id, name_id);
}

fn read_unload_class(channel: &mut HprofChannel) {
    let class_ser_num = channel.read_u32();
    // let class_id = class_ser_num_2_id.get(&class_ser_num);
    // if class_id.is_some() {
    //     class_2_name.remove(class_id.unwrap());
    // }
}

fn read_frame(channel: &mut HprofChannel, bar: &ProgressBar) {
    let frame_id = read_id(channel);
    let method_name = read_id(channel);
    let method_sig = read_id(channel);
    let src_file = read_id(channel);
    bar.inc((4 * unsafe { ID_SIZE }) as u64);
    let class_ser_num = channel.read_u32();
    let line_nr = channel.read_u32();
    bar.inc(8)
}

fn read_version(channel: &mut HprofChannel) -> String {
    let mut v = String::new();
    let mut index = 0;
    loop {
        if index > 20 {
            panic!("不支持的文件")
        }
        let byte = channel.read_char().unwrap();
        if byte == '\0' {
            break;
        }
        v.push(byte);
    }

    if v != HPROF_HEADER_101 && v != HPROF_HEADER_102 {
        panic!("不支持的版本: {}", v)
    };

    return v;
}

fn read_id(channel: &mut HprofChannel) -> u64 {
    if unsafe {
        ID_SIZE
    } == 4 {
        return channel.read_u32().unwrap() as u64;
    }
    return channel.read_u64().unwrap();
}

fn read_head(channel: &mut HprofChannel) -> (u8, u32, u32) {
    let mut tag: [u8; 1] = [0; 1];
    let count = channel.reads(&mut tag);
    if count == 0 {
        return (0, 0, 0);
    }
    let tag = tag.get(0);
    let tag = *tag.unwrap();

    let offset = channel.read_u32().unwrap();
    let length = channel.read_u32().unwrap();
    return (tag, offset, length);
}

fn read_hprof_trace(channel: &mut HprofChannel, bar: &ProgressBar) {
    let stack_trace_nr = channel.read_u32().unwrap();
    let thread_nr = channel.read_u32().unwrap();
    let frame_count = channel.read_u32().unwrap();
    bar.inc(12);
    let mut frame_ids: Vec<u64> = Vec::new();
    for _ in 0..frame_count {
        frame_ids.push(read_id(channel));
        bar.inc(unsafe { ID_SIZE } as u64)
    }
}

fn read_start_thread(channel: &mut HprofChannel, bar: &ProgressBar) {
    let threadSerialNum = channel.read_u32();
    let threadObjId = read_id(channel);
    let traceSerialNum = channel.read_u32();
    let tnameIndex = read_id(channel);
    let gnameIndex = read_id(channel);
    let pnameIndex = read_id(channel);
    let size = unsafe { ID_SIZE } as u64;
    bar.inc(4 + size + 4 + size + size + size);
}

fn read_heap_summary(channel: &mut HprofChannel, bar: &ProgressBar) {
    let live = channel.read_u32();
    let liveInst = channel.read_u32();
    let allocate = channel.read_u64();
    let allocateInst = channel.read_u64();
    bar.inc(8 + 16);
}