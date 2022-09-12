use crate::binary::{read_binary_file, read_u16, read_u32};
use crate::cp_info::{parse_cp_info, CpInfo};
use std::io::Cursor;

#[derive(Default)]
pub struct ClassFile {
    /// ClassFile Structure
    /// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    // pub methods[methods_count]: method_info,
    pub attributes_count: u16,
    // pub attributes[attributes_count]: attribute_info,
}

impl ClassFile {
    pub fn parse_from(class: &[u8]) -> ClassFile {
        let mut cursor = Cursor::new(class);
        let magic: u32 = read_u32(&mut cursor);
        let minor_version: u16 = read_u16(&mut cursor);
        let major_version: u16 = read_u16(&mut cursor);
        let constant_pool_count: u16 = read_u16(&mut cursor);
        let constant_pool = parse_cp_info(&mut cursor, constant_pool_count);
        let access_flags: u16 = read_u16(&mut cursor);
        let this_class: u16 = read_u16(&mut cursor);
        let super_class: u16 = read_u16(&mut cursor);
        let interfaces_count: u16 = read_u16(&mut cursor);
        let interfaces = parse_interfaces(&mut cursor, interfaces_count);
        let fields_count: u16 = read_u16(&mut cursor);
        let fields = parse_fields(&mut cursor, fields_count);
        let methods_count: u16;
        // let methods[methods_count]: method_info;
        let attributes_count: u16;
        // let attributes[attributes_count]: attribute_info;

        let mut class = ClassFile::default();
        class.magic = magic;
        class.minor_version = minor_version;
        class.major_version = major_version;
        class.constant_pool_count = constant_pool_count;
        class.constant_pool = constant_pool;
        class.access_flags = access_flags;
        class.this_class = this_class;
        class.super_class = super_class;
        class.interfaces_count = interfaces_count;
        class.interfaces = interfaces;
        class.fields_count = fields_count;
        class.fields = fields;
        class
    }
}

fn parse_interfaces(cursur: &mut Cursor<&[u8]>, interface_count: u16) -> Vec<u16> {
    let mut interfaces: Vec<u16> = vec![];
    for _ in 0..interface_count {
        interfaces.push(read_u16(cursur));
    }
    interfaces
}

pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

fn parse_fields(cursur: &mut Cursor<&[u8]>, fields_count: u16) -> Vec<FieldInfo> {
    if fields_count > 1 {
        panic!("fileds are not implemented yet");
    }
    vec![]
}

#[test]
fn test_parse_class() {
    // let bytes: &[u8] = &[0xCA, 0xFE, 0xBE, 0xBE];
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();

    let result = ClassFile::parse_from(binary.as_slice());

    assert_eq!(result.magic, 0xCAFEBABE_u32);
    assert_eq!(result.minor_version, 0);
    assert_eq!(result.major_version, 61);
    assert_eq!(result.constant_pool_count, 31);
    assert_eq!(
        result.constant_pool.len(),
        (result.constant_pool_count - 1) as usize
    );
    assert_eq!(result.access_flags, 0x21);
    assert_eq!(result.this_class, 14_u16);
    assert_eq!(result.super_class, 02_u16);
    assert_eq!(result.interfaces_count, 0);
    assert_eq!(result.interfaces.len(), result.interfaces_count as usize);
    assert_eq!(result.fields_count, 0);
    assert_eq!(result.fields.len(), result.fields_count as usize);
}
