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

use crate::binary::{read_binary_file, read_u16, read_u32, read_u8};
use crate::cp_info::CpInfo::{
    ConstantClassInfo, ConstantFieldref, ConstantInterfaceMethodRef, ConstantMethodRef,
};

pub enum CpInfo {
    ConstantClassInfo {
        tag: CP_TAGES,
        name_index: u16,
    },
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

    for i in 0..constant_pool_count {
        let tag = CP_TAGES::from_u8(read_u8(cursor));
        let cp_info = match tag {
            CP_TAGES::CONSTANT_Class => ConstantClassInfo {
                tag,
                name_index: read_u16(cursor),
            },

            // Method, Filed, Interface
            CP_TAGES::CONSTANT_Methodref => ConstantMethodRef {
                tag,
                class_index: read_u16(cursor),
                name_and_type_index: read_u16(cursor),
            },
            CP_TAGES::CONSTANT_Fieldref => ConstantFieldref {
                tag,
                class_index: read_u16(cursor),
                name_and_type_index: read_u16(cursor),
            },
            CP_TAGES::CONSTANT_InterfaceMethodref => ConstantInterfaceMethodRef {
                tag,
                class_index: read_u16(cursor),
                name_and_type_index: read_u16(cursor),
            },
            _ => panic!(
                "#{} the Constant Pool {:?} is not implement yet",
                i + 1,
                tag
            ),
        };
        constant_pool.push(cp_info);
    }

    constant_pool
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
    assert_eq!(result.len(), constant_pool_count as usize);
}
