use crate::binary::{read_binary_file, read_to, read_u16, read_u32};
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
        let methods_count: u16 = read_u16(&mut cursor);
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

pub struct MethodInfo {
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

fn parse_fields(cursor: &mut Cursor<&[u8]>, fields_count: u16) -> Vec<FieldInfo> {
    if fields_count > 1 {
        panic!("fileds are not implemented yet");
    }
    vec![]
}

fn parse_methods(cursor: &mut Cursor<&[u8]>, methods_count: u16) -> Vec<MethodInfo> {
    let mut methods: Vec<MethodInfo> = vec![];
    for _ in 0..methods_count {}
    methods
}

fn parse_method(cursor: &mut Cursor<&[u8]>) -> MethodInfo {
    let access_flags: u16 = read_u16(cursor);
    let name_index: u16 = read_u16(cursor);
    let descriptor_index: u16 = read_u16(cursor);
    let attributes_count: u16 = read_u16(cursor);
    let attributes = parse_attributes(cursor, attributes_count);

    MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    }
}

fn parse_attributes(cursor: &mut Cursor<&[u8]>, attributes_count: u16) -> Vec<AttributeInfo> {
    let mut attributes: Vec<AttributeInfo> = vec![];
    for _ in 0..attributes_count {
        attributes.push(parse_attribute_info(cursor))
    }
    attributes
}

fn parse_attribute_info(cursor: &mut Cursor<&[u8]>) -> AttributeInfo {
    let attribute_name_index = read_u16(cursor);
    let attribute_length = read_u32(cursor);
    AttributeInfo {
        attribute_name_index,
        attribute_length,
        info: read_to(cursor, attribute_length as usize),
    }
}

#[test]
fn test_parse_attribute_info() {
    let bytes: &[u8] = &[
        // Attributes
        0x00, 0x19, 0x00, 0x00, 0x00, 0x26, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0a, 0xb2,
        0x00, 0x07, 0xb8, 0x00, 0x0d, 0xb6, 0x00, 0x13, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1a,
        0x00, 0x00, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x09, 0x00, 0x04,
    ];
    let mut cursor = Cursor::new(bytes);

    let result = parse_attribute_info(&mut cursor);

    assert_eq!(result.attribute_name_index, 0x0019);
    assert_eq!(result.attribute_length, 0x00000026);
    assert_eq!(result.info.len(), result.attribute_length as usize);
}

#[test]
fn test_parse_method() {
    let bytes: &[u8] = &[
        0x00, 0x09, 0x00, 0x1b, 0x00, 0x1c, 0x00, 0x01, // Attributes
        0x00, 0x19, 0x00, 0x00, 0x00, 0x26, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0a, 0xb2,
        0x00, 0x07, 0xb8, 0x00, 0x0d, 0xb6, 0x00, 0x13, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1a,
        0x00, 0x00, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x09, 0x00, 0x04,
    ];
    let mut cursor = Cursor::new(bytes);

    let result = parse_method(&mut cursor);

    assert_eq!(result.access_flags, 0x09);
    assert_eq!(result.name_index, 0x1b);
    assert_eq!(result.descriptor_index, 0x1c);
    assert_eq!(result.attributes_count, 0x01);
    assert_eq!(result.attributes.len(), result.attributes_count as usize);
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
