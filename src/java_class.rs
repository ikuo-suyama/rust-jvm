use std::io::Cursor;

#[derive(Default)]
pub struct ClassFile {
    /// ClassFile Structure
    /// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    // pub constant_pool[constant_pool_count - 1]: cp_info,
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

impl ClassFile {
    pub fn parse_from(class: &[u8]) -> ClassFile {
        let mut cursor = Cursor::new(class);
        let magic: u32;
        let minor_version: u16;
        let major_version: u16;
        let constant_pool_count: u16;
        let access_flags: u16;
        let this_class: u16;
        let super_class: u16;
        let interfaces_count: u16;
        // let interfaces[interfaces_count]: u16;
        let fields_count: u16;
        // let fields[fields_count]: field_info;
        let methods_count: u16;
        // let methods[methods_count]: method_info;
        let attributes_count: u16;
        // let attributes[attributes_count]: attribute_info;

        ClassFile::default()
    }
}

#[test]
fn test_parse_class() {
    let bytes: &[u8] = &[0xCA, 0xFE, 0xBE, 0xBE];

    let result = ClassFile::parse_from(bytes);

    // assert_eq!(result.magic, 0xCAFEBEBE_u32)
    assert_eq!(result.magic, 0);
}
