use std::io::Cursor;
use std::rc::Rc;

use crate::binary::{read_binary_file, read_u16, read_u32};
use crate::class_attributes::{
    parse_attributes, parse_fields, parse_interfaces, parse_methods, AttributeInfo, FieldInfo,
    MethodInfo,
};
use crate::cp_info::{parse_cp_info, CpInfo};

#[derive(Default, Debug)]
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
    pub fields: Vec<Rc<FieldInfo>>,
    pub methods_count: u16,
    pub methods: Vec<Rc<MethodInfo>>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn parse_from(binary: &[u8]) -> ClassFile {
        let mut cursor = Cursor::new(binary);
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
        let fields = parse_fields(&mut cursor, fields_count, &constant_pool);
        let methods_count: u16 = read_u16(&mut cursor);
        let methods = parse_methods(&mut cursor, methods_count, &constant_pool);
        let attributes_count: u16 = read_u16(&mut cursor);
        let attributes = parse_attributes(&mut cursor, attributes_count, &constant_pool);

        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        }
    }
}

#[test]
fn test_parse_class() {
    // let bytes: &[u8] = &[0xCA, 0xFE, 0xBE, 0xBE];
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();

    let result = ClassFile::parse_from(binary.as_slice());

    assert_eq!(result.magic, 0xCAFEBABE_u32);
    assert_eq!(result.minor_version, 0);
    assert_eq!(result.major_version, 61);
    assert_eq!(result.constant_pool_count, 30);
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

    assert_eq!(result.methods_count, 3);
    assert_eq!(result.methods.len(), result.methods_count as usize);

    assert_eq!(result.attributes_count, 1);
    assert_eq!(result.attributes.len(), result.attributes_count as usize);
}
