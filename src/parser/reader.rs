use crate::io::channel;
use crate::io::channel::{Channel};

use std::path::PathBuf;

pub type Byte = u8;
pub type Short = u16;
pub type Int = u32;
pub type Long = u64;
pub type Result<T> = channel::Result<T>;

pub struct Reader {
    channel: Channel,
    id_size: u32,
}

impl Reader {
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let channel = Channel::open(file_path)?;
        Ok(Self { channel, id_size: 0 })
    }

    pub fn get_utf8(&mut self, len: Int) -> Result<(Long, String)> {
        // 常量池
        let length = len - self.id_size;
        let symbol_id = self.get_id()?;
        let name = self.channel.read_str(length as usize)?;
        Ok((symbol_id, name))
    }

    pub fn get_load_class(&mut self) -> Result<(Int, Long, Int, Long)> {
        // 加载类
        let serial_num = self.read_int()?;
        let class_id = self.get_id()?;
        let stack_trace_serial_no = self.read_int()?;
        let name_id = self.get_id()?;
        Ok((serial_num, class_id, stack_trace_serial_no, name_id))
    }

    pub fn get_unload_class(&mut self) -> Result<Int> {
        // 卸载类
        let class_ser_num = self.read_int()?;
        Ok(class_ser_num)
    }

    pub fn get_frame(&mut self) -> Result<(Long, Long, Long, Long, Int, Int)> {
        let frame_id = self.get_id()?;
        let method_name = self.get_id()?;
        let method_sig = self.get_id()?;
        let src_file = self.get_id()?;
        let class_ser_num = self.read_int()?;
        let line_nr = self.read_int()?;
        Ok((
            frame_id, method_name, method_sig, src_file, class_ser_num, line_nr
        ))
    }

    pub fn get_header(&mut self) -> Result<(Byte, Int, Int)> {
        let tag = self.read_byte()?;
        let offset = self.read_int()?;
        let length = self.read_int()?;
        Ok((tag, offset, length))
    }

    pub fn get_hprof_trace(&mut self) -> Result<(Int, Int, Vec<Long>)> {
        let stack_trace_nr = self.read_int()?;
        let thread_nr = self.read_int()?;
        let frame_count = self.read_int()?;
        let mut frame_ids: Vec<u64> = Vec::new();
        for _ in 0..frame_count {
            let id = self.get_id()?;
            frame_ids.push(id);
        }
        Ok((stack_trace_nr, thread_nr, frame_ids))
    }

    pub fn get_start_thread(&mut self) -> Result<(Int, Long, Int, Long, Long, Long, Int)> {
        let thread_serial_num = self.read_int()?;
        let thread_obj_id = self.get_id()?;
        let trace_serial_num = self.read_int()?;
        let t_name_index = self.get_id()?;
        let g_name_index = self.get_id()?;
        let p_name_index = self.get_id()?;
        Ok((
            thread_serial_num,
            thread_obj_id,
            trace_serial_num,
            t_name_index,
            g_name_index,
            p_name_index,
            self.id_size
        ))
    }

    pub fn get_heap_summary(&mut self) -> Result<(Int, Int, Long, Long)> {
        let live = self.read_int()?;
        let live_inst = self.read_int()?;
        let allocate = self.read_long()?;
        let allocate_inst = self.read_long()?;
        Ok((
            live, live_inst, allocate, allocate_inst
        ))
    }

    pub fn get_id(&mut self) -> Result<Long> {
        let id: Long;
        if self.id_size == 4 {
            id = self.read_int()? as Long;
        } else {
            id = self.read_long()?;
        }
        Ok(id)
    }

    pub fn get_timestamp(&mut self) -> Result<Long> {
        self.channel.read_u64()
    }

    pub fn get_id_size(&mut self) -> Result<Int> {
        if self.id_size > 0 {
            return Ok(self.id_size);
        }
        let id_size = self.read_int()?;
        self.id_size = id_size;
        Ok(id_size)
    }

    pub fn read_byte(&mut self) -> Result<Byte> {
        self.channel.read_u8()
    }

    pub fn read_int(&mut self) -> Result<Int> {
        self.channel.read_u32()
    }

    pub(crate) fn read_long(&mut self) -> Result<Long> {
        self.channel.read_u64()
    }

    pub fn read_char(&mut self) -> Result<char> {
        self.channel.read_char()
    }

    pub fn read_short(&mut self) -> Result<Short> {
        self.channel.read_u16()
    }

    pub fn skip(&mut self, len: Int) {
        self.channel.skip(len as i64);
    }

    pub fn position(&mut self) -> Result<u64> {
        self.channel.position()
    }
}

