use crate::binary::{read_to, read_u16, read_u32};
use std::io::Cursor;

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

pub fn parse_interfaces(cursur: &mut Cursor<&[u8]>, interface_count: u16) -> Vec<u16> {
    let mut interfaces: Vec<u16> = vec![];
    for _ in 0..interface_count {
        interfaces.push(read_u16(cursur));
    }
    interfaces
}

pub fn parse_fields(cursor: &mut Cursor<&[u8]>, fields_count: u16) -> Vec<FieldInfo> {
    let mut fields: Vec<FieldInfo> = vec![];
    for _ in 0..fields_count {
        fields.push(parse_field(cursor))
    }
    fields
}

fn parse_field(cursor: &mut Cursor<&[u8]>) -> FieldInfo {
    let access_flags: u16 = read_u16(cursor);
    let name_index: u16 = read_u16(cursor);
    let descriptor_index: u16 = read_u16(cursor);
    let attributes_count: u16 = read_u16(cursor);
    let attributes = parse_attributes(cursor, attributes_count);

    FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    }
}

pub fn parse_methods(cursor: &mut Cursor<&[u8]>, methods_count: u16) -> Vec<MethodInfo> {
    let mut methods: Vec<MethodInfo> = vec![];
    for _ in 0..methods_count {
        methods.push(parse_method(cursor))
    }
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

pub fn parse_attributes(cursor: &mut Cursor<&[u8]>, attributes_count: u16) -> Vec<AttributeInfo> {
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
