/* using trait
pub trait CpInfo {
    fn tag(&self) -> u8;
}

struct ConstantClassInfo {
    pub tag: u8,
    pub name_index: u16,
}

impl CpInfo for ConstantClassInfo {
    fn tag(&self) -> u8 {
        self.tag
    }
}

fn parse_cp_info(cursor: &mut Cursor<&[u8]>) -> Vec<Box<dyn CpInfo>> {
    /// here we need a [Box](https://doc.rust-jp.rs/rust-by-example-ja/std/box.html)
    /// to avoid "the trait `Sized` is not implemented for..."
    vec![Box::new(ConstantClassInfo {
        tag: 0,
        name_index: 0,
    })]
}
*/

use std::io::Cursor;

use crate::binary::{read_binary_file, read_string_to, read_u16, read_u32, read_u8};
use crate::class_file::ClassFile;
use crate::types::{JDouble, JFloat, JInteger, JLong, JString, JVMTypes};

#[derive(Debug)]
pub enum CpInfo {
    /// ConstantPool Structures
    /// [Ref](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html)
    ConstantClassInfo {
        tag: CP_TAGES,
        name_index: u16,
    },

    // Field, Method, Interface
    ConstantFieldref {
        tag: CP_TAGES,
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantMethodRef {
        tag: CP_TAGES,
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantInterfaceMethodRef {
        tag: CP_TAGES,
        class_index: u16,
        name_and_type_index: u16,
    },

    // NameAndType
    ConstantNameAndType {
        tag: CP_TAGES,
        name_index: u16,
        descriptor_index: u16,
    },

    // Primitives
    ConstantInteger {
        tag: CP_TAGES,
        bytes: u32,
    },
    ConstantFloat {
        tag: CP_TAGES,
        bytes: u32,
    },
    ConstantLong {
        tag: CP_TAGES,
        high_bytes: u32,
        low_bytes: u32,
    },
    ConstantDouble {
        tag: CP_TAGES,
        high_bytes: u32,
        low_bytes: u32,
    },

    // Utf8
    ConstantUtf8 {
        tag: CP_TAGES,
        length: u16,
        bytes: String,
    },

    // Special Value
    // Long, Double uses 2 indexes. represent this, put empty val to i+1 index
    ConstantNull,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CP_TAGES {
    CONSTANT_Class = 7,
    CONSTANT_Fieldref = 9,
    CONSTANT_Methodref = 10,
    CONSTANT_InterfaceMethodref = 11,
    CONSTANT_String = 8,
    CONSTANT_Integer = 3,
    CONSTANT_Float = 4,
    CONSTANT_Long = 5,
    CONSTANT_Double = 6,
    CONSTANT_NameAndType = 12,
    CONSTANT_Utf8 = 1,
    CONSTANT_MethodHandle = 15,
    CONSTANT_MethodType = 16,
    CONSTANT_Dynamic = 17,
    CONSTANT_InvokeDynamic = 18,
    CONSTANT_Module = 19,
    CONSTANT_Package = 20,
}

impl CP_TAGES {
    fn from_u8(value: u8) -> CP_TAGES {
        match value {
            7 => CP_TAGES::CONSTANT_Class,
            9 => CP_TAGES::CONSTANT_Fieldref,
            10 => CP_TAGES::CONSTANT_Methodref,
            11 => CP_TAGES::CONSTANT_InterfaceMethodref,
            8 => CP_TAGES::CONSTANT_String,
            3 => CP_TAGES::CONSTANT_Integer,
            4 => CP_TAGES::CONSTANT_Float,
            5 => CP_TAGES::CONSTANT_Long,
            6 => CP_TAGES::CONSTANT_Double,
            12 => CP_TAGES::CONSTANT_NameAndType,
            1 => CP_TAGES::CONSTANT_Utf8,
            15 => CP_TAGES::CONSTANT_MethodHandle,
            16 => CP_TAGES::CONSTANT_MethodType,
            17 => CP_TAGES::CONSTANT_Dynamic,
            18 => CP_TAGES::CONSTANT_InvokeDynamic,
            19 => CP_TAGES::CONSTANT_Module,
            20 => CP_TAGES::CONSTANT_Package,
            _ => panic!("the value {} is not cp_tag", value),
        }
    }
}

pub fn parse_cp_info(cursor: &mut Cursor<&[u8]>, constant_pool_count: u16) -> Vec<CpInfo> {
    let mut constant_pool: Vec<CpInfo> = vec![];

    let mut i = 1;
    while i < constant_pool_count {
        let tag = CP_TAGES::from_u8(read_u8(cursor));
        let cp_info = match tag {
            CP_TAGES::CONSTANT_Class => CpInfo::ConstantClassInfo {
                tag,
                name_index: read_u16(cursor),
            },

            // Method, Filed, Interface
            CP_TAGES::CONSTANT_Methodref => CpInfo::ConstantMethodRef {
                tag,
                class_index: read_u16(cursor),
                name_and_type_index: read_u16(cursor),
            },
            CP_TAGES::CONSTANT_Fieldref => CpInfo::ConstantFieldref {
                tag,
                class_index: read_u16(cursor),
                name_and_type_index: read_u16(cursor),
            },
            CP_TAGES::CONSTANT_InterfaceMethodref => CpInfo::ConstantInterfaceMethodRef {
                tag,
                class_index: read_u16(cursor),
                name_and_type_index: read_u16(cursor),
            },

            // NameAndType
            CP_TAGES::CONSTANT_NameAndType => CpInfo::ConstantNameAndType {
                tag,
                name_index: read_u16(cursor),
                descriptor_index: read_u16(cursor),
            },

            // Primitives
            CP_TAGES::CONSTANT_Integer => CpInfo::ConstantInteger {
                tag,
                bytes: read_u32(cursor),
            },
            CP_TAGES::CONSTANT_Float => CpInfo::ConstantFloat {
                tag,
                bytes: read_u32(cursor),
            },
            CP_TAGES::CONSTANT_Long => CpInfo::ConstantLong {
                tag,
                high_bytes: read_u32(cursor),
                low_bytes: read_u32(cursor),
            },
            CP_TAGES::CONSTANT_Double => CpInfo::ConstantDouble {
                tag,
                high_bytes: read_u32(cursor),
                low_bytes: read_u32(cursor),
            },

            // Utf8
            CP_TAGES::CONSTANT_Utf8 => {
                let length = read_u16(cursor);
                CpInfo::ConstantUtf8 {
                    tag,
                    length,
                    bytes: read_string_to(cursor, length as usize),
                }
            }
            _ => panic!(
                "#{} the Constant Pool {:?} is not implement yet",
                i + 1,
                tag
            ),
        };

        // here, CP_Double and CP_Long consumes 2 indexes
        match cp_info {
            CpInfo::ConstantDouble { .. } | CpInfo::ConstantLong { .. } => {
                println!("[DEBUG] -- Load CP: #{} = {:?}", i, cp_info);
                constant_pool.push(cp_info);
                constant_pool.push(CpInfo::ConstantNull);
                i = i + 2;
            }
            _ => {
                println!("[DEBUG] -- Load CP: #{} = {:?}", i, cp_info);
                constant_pool.push(cp_info);
                i = i + 1;
            }
        }
    }

    constant_pool
}

pub fn constant_pool_value_at(constant_pool: &Vec<CpInfo>, index: u16) -> JVMTypes {
    let parse_cp_value = |cp: &CpInfo| match cp {
        CpInfo::ConstantInteger { bytes, .. } => JVMTypes::create_integer(bytes),

        CpInfo::ConstantLong {
            high_bytes,
            low_bytes,
            ..
        } => JVMTypes::create_long(high_bytes, low_bytes),

        CpInfo::ConstantFloat { bytes, .. } => JVMTypes::create_float(bytes),

        CpInfo::ConstantDouble {
            high_bytes,
            low_bytes,
            ..
        } => JVMTypes::create_double(high_bytes, low_bytes),

        CpInfo::ConstantNull => JVMTypes::JNull,

        _ => {
            let value = constant_pool_value_as_string(constant_pool, index);
            JVMTypes::JString(JString { value })
        }
    };

    let cp_not_found_error = |index: u16| {
        panic!(
            "index out of bounds for constant pool: length {} index {}",
            constant_pool.len(),
            index
        )
    };

    let maybe_cp = constant_pool.get((index - 1) as usize);
    let value = match maybe_cp {
        Some(cp) => parse_cp_value(cp),
        None => cp_not_found_error(index),
    };
    value
}

pub fn constant_pool_value_as_string(constant_pool: &Vec<CpInfo>, index: u16) -> String {
    let parse_cp_value = |cp: &CpInfo| match cp {
        CpInfo::ConstantUtf8 { bytes, .. } => bytes.clone(),
        CpInfo::ConstantClassInfo { name_index, .. } => {
            constant_pool_value_as_string(constant_pool, name_index.clone())
        }
        CpInfo::ConstantNameAndType {
            name_index,
            descriptor_index,
            ..
        } => {
            let name = constant_pool_value_as_string(constant_pool, name_index.clone());
            let desc = constant_pool_value_as_string(constant_pool, descriptor_index.clone());
            format!("{}:{}", name, desc)
        }
        CpInfo::ConstantMethodRef {
            class_index,
            name_and_type_index,
            ..
        } => {
            let class = constant_pool_value_as_string(constant_pool, class_index.clone());
            let nt = constant_pool_value_as_string(constant_pool, name_and_type_index.clone());
            format!("{}.{}", class, nt)
        }
        CpInfo::ConstantFieldref {
            class_index,
            name_and_type_index,
            ..
        } => {
            let class = constant_pool_value_as_string(constant_pool, class_index.clone());
            let nt = constant_pool_value_as_string(constant_pool, name_and_type_index.clone());
            format!("{}.{}", class, nt)
        }
        _ => panic!("not String type CP passed. cp: {:#?}", cp),
    };
    let cp_not_found_error = |index: u16| {
        panic!(
            "index out of bounds for constant pool: length {} index {}",
            constant_pool.len(),
            index
        )
    };

    let maybe_cp = constant_pool.get((index - 1) as usize);
    let value = match maybe_cp {
        Some(cp) => parse_cp_value(cp),
        None => cp_not_found_error(index),
    };
    value
}

#[test]
fn test_parse_cp_info() {
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();
    let mut cursor = Cursor::new(binary.as_slice());

    // skip consts
    let _ = read_u32(&mut cursor);
    let _ = read_u16(&mut cursor);
    let _ = read_u16(&mut cursor);
    let constant_pool_count = read_u16(&mut cursor);

    let result = parse_cp_info(&mut cursor, constant_pool_count);
    assert_eq!(result.len(), (constant_pool_count - 1) as usize);
}

#[test]
fn test_constant_pool_value_at() {
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();
    let class_file = ClassFile::parse_from(binary.as_slice());

    // for cp index, see @sampleSum.jvm file
    let cp = class_file.constant_pool;
    let utf8 = constant_pool_value_as_string(&cp, 29);
    assert_eq!(utf8, "SimpleSum.java");

    let class = constant_pool_value_as_string(&cp, 2);
    assert_eq!(class, "java/lang/Object");

    let name_and_type = constant_pool_value_as_string(&cp, 3);
    assert_eq!(name_and_type, "<init>:()V");

    let method_ref = constant_pool_value_as_string(&cp, 1);
    assert_eq!(method_ref, "java/lang/Object.<init>:()V");

    let field_ref = constant_pool_value_as_string(&cp, 7);
    assert_eq!(field_ref, "java/lang/System.out:Ljava/io/PrintStream;");
}

#[test]
fn test_constant_pool_value_static() {
    let binary = read_binary_file(&"java/Types.class".to_owned()).unwrap();
    let class_file = ClassFile::parse_from(binary.as_slice());

    // for cp index, see @sampleSum.jvm file
    let cp = class_file.constant_pool;

    let int = constant_pool_value_at(&cp, 7);
    let expected = JInteger {
        value: 1001001001,
        bytes: 1001001001 as u32,
    };
    assert_eq!(int, JVMTypes::JInteger(expected));

    let long = constant_pool_value_at(&cp, 11);
    let expected = JLong {
        value: 20202020202020 as i64,
        high_bytes: 4703 as u32,
        low_bytes: 2789008932 as u32,
    };
    assert_eq!(long, JVMTypes::JLong(expected));

    let float = constant_pool_value_at(&cp, 13);
    let expected = JFloat {
        // value: 1.01 as f32,
        value: 1.01 as f32,
        bytes: 1065437102,
    };
    assert_eq!(float, JVMTypes::JFloat(expected));

    let double = constant_pool_value_at(&cp, 15);
    let expected = JDouble {
        value: 1.0e100 as f64,
        high_bytes: 1420970413,
        low_bytes: 630506365,
    };
    assert_eq!(double, JVMTypes::JDouble(expected));
}

#[test]
#[should_panic]
fn test_constant_pool_value_static_to_string() {
    let binary = read_binary_file(&"java/Types.class".to_owned()).unwrap();
    let class_file = ClassFile::parse_from(binary.as_slice());

    // for cp index, see @sampleSum.jvm file
    let cp = class_file.constant_pool;
    let utf8 = constant_pool_value_as_string(&cp, 9);
    assert_eq!(utf8, "SimpleSum.java");
}

#[test]
#[should_panic]
fn test_constant_pool_value_obe() {
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();
    let class_file = ClassFile::parse_from(binary.as_slice());
    let cp = class_file.constant_pool;

    let _ = constant_pool_value_as_string(&cp, 31);
}
