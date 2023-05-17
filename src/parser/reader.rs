use crate::io::channel;
use crate::io::channel::{Channel, Result, Byte, Short, Int, Long};
use std::path::PathBuf;
use derive_getters::Getters;

/// # Hprof File Reader
pub struct Reader {
    ///
    channel: Channel,
    id_size: u32,
}

/// # A section from the hprof file
pub trait Section {
    fn read(reader: &mut Reader, len: Int) -> Self;
}

#[derive(Clone, Debug, Getters)]
pub struct Utf8 {
    symbol_id: Long,
    name: String,
}

impl Section for Utf8 {
    fn read(reader: &mut Reader, len: Int) -> Self {
        // 常量池
        let length = len - reader.id_size;
        let symbol_id = reader.get_id();
        let name = reader.channel.read_str(length as usize).unwrap();
        Utf8 {
            symbol_id,
            name,
        }
    }
}

#[derive(Clone, Debug, Getters)]
pub struct LoadedClass {
    serial_num: Int,
    class_id: Long,
    stack_trace_ser: Int,
    name_id: Long,
}

impl Section for LoadedClass {
    fn read(reader: &mut Reader, _: Int) -> Self {
        // 常量池
        let serial_num = reader.read_int();
        let class_id = reader.get_id();
        let stack_trace_ser = reader.read_int();
        let name_id = reader.get_id();
        LoadedClass {
            serial_num,
            class_id,
            stack_trace_ser,
            name_id,
        }
    }
}

#[derive(Clone, Debug, Getters)]
pub struct UnLoadClass {
    class_ser_num: Int,
}

impl Section for UnLoadClass {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let class_ser_num = reader.read_int();
        UnLoadClass { class_ser_num }
    }
}

#[derive(Clone, Debug, Getters)]
pub struct Frame {
    id: Long,
    method_name: Long,
    method_sig: Long,
    src_file: Long,
    class_ser_num: Int,
    line_nr: Int,
}

impl Section for Frame {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let frame_id = reader.get_id();
        let method_name = reader.get_id();
        let method_sig = reader.get_id();
        let src_file = reader.get_id();
        let class_ser_num = reader.read_int();
        let line_nr = reader.read_int();
        Frame {
            id: frame_id,
            method_name,
            method_sig,
            src_file,
            class_ser_num,
            line_nr,
        }
    }
}

#[derive(Clone, Debug, Getters)]
pub struct Trace {
    stack_trace_nr: Int,
    thread_nr: Int,
    frame_ids: Vec<u64>,
}

impl Section for Trace {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let stack_trace_nr = reader.read_int();
        let thread_nr = reader.read_int();
        let frame_count = reader.read_int();
        let mut frame_ids: Vec<u64> = Vec::new();
        for _ in 0..frame_count {
            let id = reader.get_id();
            frame_ids.push(id);
        }
        Trace {
            stack_trace_nr,
            thread_nr,
            frame_ids,
        }
    }
}

#[derive(Clone, Debug, Getters)]
pub struct StartThread {
    thread_serial_num: Int,
    thread_obj_id: Long,
    trace_serial_num: Int,
    t_name_index: Long,
    g_name_index: Long,
    p_name_index: Long,
}

impl Section for StartThread {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let thread_serial_num = reader.read_int();
        let thread_obj_id = reader.get_id();
        let trace_serial_num = reader.read_int();
        let t_name_index = reader.get_id();
        let g_name_index = reader.get_id();
        let p_name_index = reader.get_id();
        StartThread {
            trace_serial_num,
            thread_obj_id,
            thread_serial_num,
            t_name_index,
            g_name_index,
            p_name_index,
        }
    }
}

#[derive(Clone, Debug, Getters)]
pub struct HeapSummary {
    live: Int,
    live_inst: Int,
    allocate: Long,
    allocate_inst: Long,
}

impl Section for HeapSummary {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let live = reader.read_int();
        let live_inst = reader.read_int();
        let allocate = reader.read_long();
        let allocate_inst = reader.read_long();
        HeapSummary {
            live,
            live_inst,
            allocate,
            allocate_inst,
        }
    }
}

impl Reader {
    pub fn new(file_path: &PathBuf) -> Result<Self> {
        let channel = Channel::open(&file_path)?;
        Ok(Self { channel, id_size: 0 })
    }

    pub fn read<T: Section>(&mut self, len: Int) -> T {
        T::read(self, len)
    }

    pub fn get_header(&mut self) -> Result<(Byte, Int, Int)> {
        let tag = self.channel.read_byte()?;
        let offset = self.read_int();
        let length = self.read_int();
        Ok((tag, offset, length))
    }

    pub fn get_id(&mut self) -> Long {
        if self.id_size == 4 {
            self.read_int() as Long
        } else {
            self.read_long()
        }
    }

    pub fn get_timestamp(&mut self) -> Result<Long> {
        self.channel.read_long()
    }

    pub fn get_id_size(&mut self) -> Result<Int> {
        if self.id_size > 0 {
            return Ok(self.id_size);
        }
        let id_size = self.read_int();
        self.id_size = id_size;
        Ok(id_size)
    }

    pub fn read_byte(&mut self) -> Byte {
        self.channel.read_byte().unwrap()
    }

    pub fn read_int(&mut self) -> Int {
        self.channel.read_int().unwrap()
    }

    pub fn read_long(&mut self) -> Long {
        self.channel.read_long().unwrap()
    }

    pub fn read_char(&mut self) -> Result<char> {
        self.channel.read_char()
    }

    pub fn read_short(&mut self) -> Result<Short> {
        self.channel.read_short()
    }

    pub fn skip(&mut self, len: Int) {
        self.channel.skip(len as i64);
    }

    pub fn position(&mut self) -> Result<u64> {
        self.channel.position()
    }
}

