use crate::binary::{read_binary_file, read_u16, read_u32};
use std::io::Cursor;

#[derive(Default)]
pub struct ClassFile {
    /// ClassFile Structure
    /// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<Box<dyn CpInfo>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    // pub interfaces[interfaces_count]: u16,
    pub fields_count: u16,
    // pub fields[fields_count]: field_info,
    pub methods_count: u16,
    // pub methods[methods_count]: method_info,
    pub attributes_count: u16,
    // pub attributes[attributes_count]: attribute_info,
}

// TODO
// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
// should use enum instead of trait?
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

impl ClassFile {
    pub fn parse_from(class: &[u8]) -> ClassFile {
        let mut cursor = Cursor::new(class);
        let magic: u32 = read_u32(&mut cursor);
        let minor_version: u16 = read_u16(&mut cursor);
        let major_version: u16 = read_u16(&mut cursor);
        let constant_pool_count: u16 = read_u16(&mut cursor);
        let constant_pool = parse_cp_info(&mut cursor);
        let access_flags: u16 = read_u16(&mut cursor);
        let this_class: u16 = read_u16(&mut cursor);
        let super_class: u16 = read_u16(&mut cursor);
        let interfaces_count: u16 = read_u16(&mut cursor);
        // let interfaces[interfaces_count]: u16;
        let fields_count: u16;
        // let fields[fields_count]: field_info;
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
        class
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

#[test]
fn test_parse_class() {
    // let bytes: &[u8] = &[0xCA, 0xFE, 0xBE, 0xBE];
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();

    let result = ClassFile::parse_from(binary.as_slice());

    assert_eq!(result.magic, 0xCAFEBABE_u32);
    assert_eq!(result.minor_version, 0);
    assert_eq!(result.major_version, 61);
    assert_eq!(result.constant_pool_count, 31);
    assert_eq!(result.constant_pool.get(0).unwrap().tag(), 0);
    assert_eq!(result.access_flags, 2560);
    assert_eq!(result.this_class, 0x12_u16);
    assert_eq!(result.super_class, 0x12_u16);
    assert_eq!(result.interfaces_count, 0x12_u16);
}
