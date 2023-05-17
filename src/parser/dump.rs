use std::fmt::format;
use std::io::{Error, ErrorKind};
use std::process::id;
use crate::parser::reader::{Reader};
use crate::io::channel::{Byte, Int, Long, Result, Short};

const HPROF_GC_ROOT_UNKNOWN: u8 = 0xff;
const HPROF_GC_ROOT_JNI_GLOBAL: u8 = 0x01;
const HPROF_GC_ROOT_JNI_LOCAL: u8 = 0x02;
const HPROF_GC_ROOT_JAVA_FRAME: u8 = 0x03;
const HPROF_GC_ROOT_NATIVE_STACK: u8 = 0x04;
const HPROF_GC_ROOT_STICKY_CLASS: u8 = 0x05;
const HPROF_GC_ROOT_THREAD_BLOCK: u8 = 0x06;
const HPROF_GC_ROOT_MONITOR_USED: u8 = 0x07;
const HPROF_GC_ROOT_THREAD_OBJ: u8 = 0x08;

pub struct Dump {}

pub fn get_heap_dump(reader: &mut Reader, len: Int) -> Result<Dump> {
    for i in 0..len {
        let tag = reader.read_byte();
        match tag {
            HPROF_GC_ROOT_UNKNOWN => {
                let id = reader.get_id();
            }
            HPROF_GC_ROOT_THREAD_OBJ => {
                let id = reader.get_id();
                let thread_seq = reader.read_int();
                let stack_seq = reader.read_int();
            }
            HPROF_GC_ROOT_JNI_GLOBAL => {
                let id = reader.get_id();
                let global_ref_id = reader.get_id();
            }
            HPROF_GC_ROOT_JNI_LOCAL => {
                let id = reader.get_id();
                let thread_seq = reader.read_int();
                let depth = reader.read_int();
            }
            HPROF_GC_ROOT_JAVA_FRAME => {
                let id = reader.get_id();
                let thread_seq = reader.read_int();
                let depth = reader.read_int();
            }
            HPROF_GC_ROOT_NATIVE_STACK => {
                let id = reader.get_id();
                let thread_seq = reader.read_int();
            }
            HPROF_GC_ROOT_STICKY_CLASS => {
                let id = reader.get_id();
            }
            HPROF_GC_ROOT_THREAD_BLOCK => {
                let id = reader.get_id();
                let thread_seq = reader.read_int();
            }
            HPROF_GC_ROOT_MONITOR_USED => {
                let id = reader.get_id();
            }
            HPROF_GC_CLASS_DUMP => {
                reader.skip(len);
            }
        }
    }
    Ok(Dump {})
}

struct Class {
    id: Long,
    super_id: Long,
    loader_id: Long,

}

fn read_class(reader: &mut Reader) -> Result<Class> {
    let id = reader.get_id();
    let stack_ser = reader.read_int();
    let super_id = reader.get_id();
    let class_loader_id = reader.get_id();

    // read signers, protection domain, reserved ids (2)
    let signers_id = reader.get_id();
    let prot_domain_id = reader.get_id();
    let reserved1 = reader.get_id();
    let reserved12 = reader.get_id();

    let inst_size = reader.read_int();

    let const_pool_size = reader.read_short();
    for _ in 0..const_pool_size {
        let index = reader.read_short();
        let name = format!("<constant pool[{}]>", index);
        let t = reader.read_byte();

        let val = get_value(reader, t);
    }

    let num_static_fields = reader.read_short();
    for _ in 0..num_static_fields {
        let name_id = reader.get_id();
        let t = reader.read_byte();
        let val = get_value(reader, t);
    }

    let num_inst_fields = reader.read_short();
    for _ in 0..num_inst_fields {
        let name_id = reader.get_id();
        let t = reader.read_byte();
    }
    
    Ok(Class {
        id: 0,
        super_id: 0,
        loader_id: 0,
    })
}

const OBJECT_TYPE: u8 = 2;
const BOOLEAN_TYPE: u8 = 4;
const CHAR_TYPE: u8 = 5;
const FLOAT_TYPE: u8 = 6;
const DOUBLE_TYPE: u8 = 7;
const BYTE_TYPE: u8 = 8;
const SHORT_TYPE: u8 = 9;
const INT_TYPE: u8 = 10;
const LONG_TYPE: u8 = 11;

enum Value {
    Id(Long),
    Bool(bool),
    Char(char),
    Float(u32),
    Double(u64),
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
}

fn get_value(reader: &mut Reader, t: Byte) -> Result<Value> {
    return match t {
        OBJECT_TYPE => {
            let id = reader.get_id();
            Ok(Value::Id(id))
        }
        BOOLEAN_TYPE => {
            let byte = reader.read_byte();
            Ok(Value::Bool(byte != 0))
        }
        CHAR_TYPE => {
            let char = reader.read_char();
            Ok(Value::Char(char))
        }
        FLOAT_TYPE => {
            let float = reader.read_int();
            Ok(Value::Float(float))
        }
        DOUBLE_TYPE => {
            let double = reader.read_long();
            Ok(Value::Double(double))
        }
        BYTE_TYPE => {
            let byte = reader.read_byte();
            Ok(Value::Byte(byte))
        }
        SHORT_TYPE => {
            let short = reader.read_short();
            Ok(Value::Short(short))
        }
        INT_TYPE => {
            let int = reader.read_int();
            Ok(Value::Int(int))
        }
        LONG_TYPE => {
            let long = reader.read_long();
            Ok(Value::Long(long))
        }
        _ => {
            Err(Error::new(ErrorKind::InvalidData,
                           format!("无效的类型: {}，position {}", t, reader.position().unwrap())))
        }
    };
}

