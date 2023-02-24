use std::path::Path;
use super::channel::{Byte, Channel, Int, Long, Short, Result};

pub struct ChannelWrapper {
    channel: Channel,
    id_size: u32,
}

impl ChannelWrapper {
    pub fn wrapper(file_path: &Path) -> Result<ChannelWrapper> {
        let channel = Channel::new(file_path)?;
        Ok(
            ChannelWrapper { channel, id_size: 0 }
        )
    }
    pub fn new(channel: Channel) -> ChannelWrapper {
        ChannelWrapper {
            channel,
            id_size: 0,
        }
    }
    pub fn read_utf8(&mut self, len: Int) -> Result<(Long, String)> {
        // 常量池
        let length = len - self.id_size;
        let symbol_id = self.read_id()?;
        let name = self.channel.read_str(length as usize)?;
        Ok((symbol_id, name))
    }

    pub fn read_load_class(&mut self) -> Result<(Int, Long, Long)> {
        // 加载类
        let serial_num = self.channel.read_int()?;
        let class_id = self.read_id()?;
        self.channel.skip(4);
        let name_id = self.read_id()?;
        Ok((serial_num, class_id, name_id))
    }

    pub fn read_unload_class(&mut self) -> Result<Int> {
        // 卸载类
        let class_ser_num = self.channel.read_int()?;
        Ok(class_ser_num)
    }

    pub fn read_frame(&mut self) -> Result<(Long, Long, Long, Long, Int, Int)> {
        let frame_id = self.read_id()?;
        let method_name = self.read_id()?;
        let method_sig = self.read_id()?;
        let src_file = self.read_id()?;
        let class_ser_num = self.channel.read_int()?;
        let line_nr = self.channel.read_int()?;
        Ok((
            frame_id, method_name, method_sig, src_file, class_ser_num, line_nr
        ))
    }

    pub fn read_header(&mut self) -> Result<(Byte, Int, Int)> {
        let tag = self.channel.read_byte()?;
        let offset = self.channel.read_int()?;
        let length = self.channel.read_int()?;
        Ok((tag, offset, length))
    }

    pub fn read_hprof_trace(&mut self) -> Result<(Int, Int, Vec<Long>)> {
        let stack_trace_nr = self.channel.read_int()?;
        let thread_nr = self.channel.read_int()?;
        let frame_count = self.channel.read_int()?;
        let mut frame_ids: Vec<u64> = Vec::new();
        for _ in 0..frame_count {
            let id = self.read_id()?;
            frame_ids.push(id);
        }
        Ok((stack_trace_nr, thread_nr, frame_ids))
    }

    pub fn read_start_thread(&mut self) -> Result<(Int, Long, Int, Long, Long, Long, Int)> {
        let thread_serial_num = self.channel.read_int()?;
        let thread_obj_id = self.read_id()?;
        let trace_serial_num = self.channel.read_int()?;
        let t_name_index = self.read_id()?;
        let g_name_index = self.read_id()?;
        let p_name_index = self.read_id()?;
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

    pub fn read_heap_summary(&mut self) -> Result<(Int, Int, Long, Long)> {
        let live = self.channel.read_int()?;
        let live_inst = self.channel.read_int()?;
        let allocate = self.channel.read_long()?;
        let allocate_inst = self.channel.read_long()?;
        Ok((
            live, live_inst, allocate, allocate_inst
        ))
    }

    pub fn read_id(&mut self) -> Result<Long> {
        let id: Long;
        if self.id_size == 4 {
            id = self.channel.read_int()? as Long;
        } else {
            id = self.channel.read_long()?;
        }
        Ok(id)
    }

    pub fn read_char(&mut self) -> Result<char> {
        self.channel.read_char()
    }

    pub fn read_long(&mut self) -> Result<Long> {
        self.channel.read_long()
    }

    pub fn read_int(&mut self) -> Result<Int> {
        self.channel.read_int()
    }

    pub fn read_id_size(&mut self) -> Result<u32> {
        if self.id_size > 0 {
            return Ok(self.id_size);
        }
        let id_size = self.channel.read_int()?;
        self.id_size = id_size;
        Ok(id_size)
    }

    pub fn skip(&mut self, len: u32) {
        self.channel.skip(len as i64)
    }
}