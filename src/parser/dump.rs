use std::fmt::format;
use std::io::{Error, ErrorKind};
use std::process::id;
use derive_getters::Getters;
use crate::parser::reader::{Reader, Section};
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

const HPROF_GC_CLASS_DUMP: u8 = 0x20;
const HPROF_GC_INSTANCE_DUMP: u8 = 0x21;
const HPROF_GC_OBJ_ARRAY_DUMP: u8 = 0x22;
const HPROF_GC_PRIM_ARRAY_DUMP: u8 = 0x23;

pub struct Dump {}

pub fn get_heap_dump(reader: &mut Reader, len: Int) -> Result<Dump> {
    for i in 0..len {
        let tag = reader.read_byte();
        match tag {
            HPROF_GC_ROOT_UNKNOWN => {
                // unknown root
                let obj_id = reader.get_id();
            }
            HPROF_GC_ROOT_THREAD_OBJ => {
                // thread object
                let thread_obj_id = reader.get_id();    // thread object ID  (may be 0 for a thread newly attached through JNI)
                let thread_seq = reader.read_int(); // thread sequence number
                let stack_seq = reader.read_int();  // stack trace sequence number
                println!("gc-root, thread {}", thread_obj_id)
            }
            HPROF_GC_ROOT_JNI_GLOBAL => {
                // JNI global ref root
                let obj_id = reader.get_id();
                let global_ref_id = reader.get_id();    // JNI global ref ID
            }
            HPROF_GC_ROOT_JNI_LOCAL => {
                // JNI local ref
                let obj_id = reader.get_id();
                let thread_seq = reader.read_int(); // thread serial number
                let depth = reader.read_int();  // frame # in stack trace (-1 for empty)
            }
            HPROF_GC_ROOT_JAVA_FRAME => {
                // Java stack frame
                let obj_id = reader.get_id();
                let thread_seq = reader.read_int(); // thread serial number
                let depth = reader.read_int(); // frame # in stack trace (-1 for empty)
            }
            HPROF_GC_ROOT_NATIVE_STACK => {
                // Native stack
                let obj_id = reader.get_id();
                let thread_seq = reader.read_int(); // thread serial number
            }
            HPROF_GC_ROOT_STICKY_CLASS => {
                // System class
                let obj_id = reader.get_id();
            }
            HPROF_GC_ROOT_THREAD_BLOCK => {
                // Reference from thread block
                let obj_id = reader.get_id();
                let thread_seq = reader.read_int(); // thread serial number
                println!("gc-root, thread {}", obj_id);
            }
            HPROF_GC_ROOT_MONITOR_USED => {
                // Busy monitor
                let obj_id = reader.get_id();
            }
            HPROF_GC_CLASS_DUMP => {
                // dump of a class object
                let class_dump = reader.read::<ClassObject>(len);
                println!("class {}", class_dump.id);
            }
            HPROF_GC_INSTANCE_DUMP => {
                // dump of a class object
                let normal_obj = reader.read::<NormalObject>(len);
                println!("obj {}", normal_obj.id);
            }
            HPROF_GC_OBJ_ARRAY_DUMP => {
                let obj_array = reader.read::<ObjectArray>(len);
            }
            HPROF_GC_PRIM_ARRAY_DUMP => {
                // Todo let prim_array = reader.read::<PrimitiveArray>(len);
                reader.skip(len - 1);
            }
            _ => {
                reader.skip(len - 1);
            }
        }
    }
    Ok(Dump {})
}

/// # dump of a class object
#[derive(Clone, Debug, Getters)]
struct ClassObject {
    id: Long,
    super_id: Long,
    class_loader_id: Long,

    signers_id: Long,
}

impl Section for ClassObject {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let id = reader.get_id();
        let stack_ser = reader.read_int();
        let super_id = reader.get_id();
        let class_loader_id = reader.get_id();

        // read signers, protection domain, reserved ids (2)
        let signers_id = reader.get_id();
        let protection_domain_id = reader.get_id();
        let reserved1 = reader.get_id();
        let reserved12 = reader.get_id();

        // instance size (in bytes)
        let inst_size = reader.read_int();

        // size of constant pool
        let const_pool_size = reader.read_short();
        for _ in 0..const_pool_size {
            let index = reader.read_short();    // constant pool index
            let name = format!("<constant pool[{}]>", index);
            let ty = reader.read_byte();    // type
            let val = get_value(reader, ty);
        }

        // number of static fields
        let num_static_fields = reader.read_short();
        for _ in 0..num_static_fields {
            let name_id = reader.get_id();  // static field name
            let ty = reader.read_byte();    // type
            let val = get_value(reader, ty);
        }

        // number of inst. fields (not inc. super)
        let num_inst_fields = reader.read_short();
        for _ in 0..num_inst_fields {
            let name_id = reader.get_id();  // instance field name
            let ty = reader.read_byte();    // type
        }

        ClassObject {
            id,
            super_id,
            class_loader_id,

            signers_id,
        }
    }
}

/// # dump of a normal object
#[derive(Clone, Debug, Getters)]
struct NormalObject {
    /// object ID
    id: Long,
    /// stack trace serial number
    stack_trace_ser: Int,
    /// class object ID
    class_id: Long,
}

impl Section for NormalObject {
    fn read(reader: &mut Reader, _: Int) -> Self {
        let id = reader.get_id();
        let stack_trace_id = reader.read_int();
        let class_id = reader.get_id();
        // number of bytes that follow
        let payload = reader.read_int();
        // instance field values (class, followed by super, super's super ...)
        // Todo 暂时跳过
        reader.skip(payload);
        NormalObject {
            id,
            stack_trace_ser: stack_trace_id,
            class_id,
        }
    }
}

/// # dump of an object array
#[derive(Clone, Debug, Getters)]
struct ObjectArray {
    /// object ID
    id: Long,
    /// stack trace serial number
    stack_trace_ser: Int,
    /// number of elements
    len: Int,
    /// class object ID
    class_id: Long,
    /// elements
    elements: Vec<Long>,
}

impl Section for ObjectArray {
    fn read(reader: &mut Reader, len: Int) -> Self {
        let id = reader.get_id();
        let stack_trace_id = reader.read_int();
        let num = reader.read_int();
        let class_id = reader.get_id();
        let mut elements: Vec<Long> = Vec::new();
        for _ in 0..num {
            let element_id = reader.get_id();
            elements.push(element_id);
        }
        ObjectArray {
            id,
            stack_trace_ser: stack_trace_id,
            class_id,
            len,
            elements,
        }
    }
}

/// # dump of a primitive array
#[derive(Clone, Debug, Getters)]
struct PrimitiveArray {
    /// array object ID
    id: Long,
    /// stack trace serial number
    stack_trace_ser: Int,
    /// number of elements
    len: Int,
    /// element type
    element_type: Byte,
    /// elements
    elements: Vec<Long>,
}

impl Section for PrimitiveArray {
    fn read(reader: &mut Reader, len: Int) -> Self {
        let id = reader.get_id();
        let stack_trace_id = reader.read_int();
        let num = reader.read_int();
        let element_type = reader.read_byte();
        let mut elements: Vec<Long> = Vec::new();

        PrimitiveArray {
            id,
            stack_trace_ser: stack_trace_id,
            len,
            element_type,
            elements,
        }
    }
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

