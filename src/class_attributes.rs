use std::io::Cursor;

use crate::binary::{read_to, read_u16, read_u32};
use crate::class_attributes::PredefinedAttributes::Code;
use crate::class_file::ClassFile;
use crate::cp_info::{constant_pool_value_at, CpInfo, CP_TAGES};

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    GeneralAttributeInfo {
        attribute_name_index: u16,
        attribute_length: u32,
        info: Vec<u8>,
    },
}

pub enum PredefinedAttributes {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

impl PredefinedAttributes {
    fn from(value: &str) -> PredefinedAttributes {
        match value {
            "ConstantValue" => PredefinedAttributes::ConstantValue,
            "Code" => PredefinedAttributes::Code,
            "StackMapTable" => PredefinedAttributes::StackMapTable,
            "Exceptions" => PredefinedAttributes::Exceptions,
            "InnerClasses" => PredefinedAttributes::InnerClasses,
            "EnclosingMethod" => PredefinedAttributes::EnclosingMethod,
            "Synthetic" => PredefinedAttributes::Synthetic,
            "Signature" => PredefinedAttributes::Signature,
            "SourceFile" => PredefinedAttributes::SourceFile,
            "SourceDebugExtension" => PredefinedAttributes::SourceDebugExtension,
            "LineNumberTable" => PredefinedAttributes::LineNumberTable,
            "LocalVariableTable" => PredefinedAttributes::LocalVariableTable,
            "LocalVariableTypeTable" => PredefinedAttributes::LocalVariableTypeTable,
            "Deprecated" => PredefinedAttributes::Deprecated,
            "RuntimeVisibleAnnotations" => PredefinedAttributes::RuntimeVisibleAnnotations,
            "RuntimeInvisibleAnnotations" => PredefinedAttributes::RuntimeInvisibleAnnotations,
            "RuntimeVisibleParameterAnnotations" => {
                PredefinedAttributes::RuntimeVisibleParameterAnnotations
            }
            "RuntimeInvisibleParameterAnnotations" => {
                PredefinedAttributes::RuntimeInvisibleParameterAnnotations
            }
            "RuntimeVisibleTypeAnnotations" => PredefinedAttributes::RuntimeVisibleTypeAnnotations,
            "RuntimeInvisibleTypeAnnotations" => {
                PredefinedAttributes::RuntimeInvisibleTypeAnnotations
            }
            "AnnotationDefault" => PredefinedAttributes::AnnotationDefault,
            "BootstrapMethods" => PredefinedAttributes::BootstrapMethods,
            "MethodParameters" => PredefinedAttributes::MethodParameters,
            "Module" => PredefinedAttributes::Module,
            "ModulePackages" => PredefinedAttributes::ModulePackages,
            "ModuleMainClass" => PredefinedAttributes::ModuleMainClass,
            "NestHost" => PredefinedAttributes::NestHost,
            "NestMembers" => PredefinedAttributes::NestMembers,
            "Record" => PredefinedAttributes::Record,
            "PermittedSubclasses" => PredefinedAttributes::PermittedSubclasses,
            _ => panic!("Unsupported Attribute: {}", value),
        }
    }
}

pub fn parse_interfaces(cursur: &mut Cursor<&[u8]>, interface_count: u16) -> Vec<u16> {
    let mut interfaces: Vec<u16> = vec![];
    for _ in 0..interface_count {
        interfaces.push(read_u16(cursur));
    }
    interfaces
}

pub fn parse_fields(
    cursor: &mut Cursor<&[u8]>,
    fields_count: u16,
    cp: &Vec<CpInfo>,
) -> Vec<FieldInfo> {
    let mut fields: Vec<FieldInfo> = vec![];
    for _ in 0..fields_count {
        fields.push(parse_field(cursor, cp))
    }
    fields
}

fn parse_field(cursor: &mut Cursor<&[u8]>, cp: &Vec<CpInfo>) -> FieldInfo {
    let access_flags: u16 = read_u16(cursor);
    let name_index: u16 = read_u16(cursor);
    let descriptor_index: u16 = read_u16(cursor);
    let attributes_count: u16 = read_u16(cursor);
    let attributes = parse_attributes(cursor, attributes_count, cp);

    FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    }
}

pub fn parse_methods(
    cursor: &mut Cursor<&[u8]>,
    methods_count: u16,
    cp: &Vec<CpInfo>,
) -> Vec<MethodInfo> {
    let mut methods: Vec<MethodInfo> = vec![];
    for _ in 0..methods_count {
        methods.push(parse_method(cursor, cp))
    }
    methods
}

fn parse_method(cursor: &mut Cursor<&[u8]>, cp: &Vec<CpInfo>) -> MethodInfo {
    let access_flags: u16 = read_u16(cursor);
    let name_index: u16 = read_u16(cursor);
    let descriptor_index: u16 = read_u16(cursor);
    let attributes_count: u16 = read_u16(cursor);
    let attributes = parse_attributes(cursor, attributes_count, cp);

    MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    }
}

pub fn parse_attributes(
    cursor: &mut Cursor<&[u8]>,
    attributes_count: u16,
    cp: &Vec<CpInfo>,
) -> Vec<AttributeInfo> {
    let mut attributes: Vec<AttributeInfo> = vec![];
    for _ in 0..attributes_count {
        attributes.push(parse_attribute_info(cursor, cp))
    }
    attributes
}

fn parse_attribute_info(cursor: &mut Cursor<&[u8]>, constant_pool: &Vec<CpInfo>) -> AttributeInfo {
    let attribute_name_index = read_u16(cursor);
    let attribute_length = read_u32(cursor);

    let attribute_name = constant_pool_value_at(constant_pool, attribute_name_index);
    match PredefinedAttributes::from(attribute_name.as_str()) {
        _ => AttributeInfo::GeneralAttributeInfo {
            attribute_name_index,
            attribute_length,
            info: read_to(cursor, attribute_length as usize),
        },
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

    let result = parse_attribute_info(&mut cursor, &cp_test::dummy_cp());
    match result {
        AttributeInfo::GeneralAttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        } => {
            assert_eq!(attribute_name_index, 0x0019);
            assert_eq!(attribute_length, 0x00000026);
            assert_eq!(info.len(), attribute_length as usize);
        }
    }
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

    let result = parse_method(&mut cursor, &cp_test::dummy_cp());

    assert_eq!(result.access_flags, 0x09);
    assert_eq!(result.name_index, 0x1b);
    assert_eq!(result.descriptor_index, 0x1c);
    assert_eq!(result.attributes_count, 0x01);
    assert_eq!(result.attributes.len(), result.attributes_count as usize);
}

#[cfg(test)]
mod cp_test {
    use crate::cp_info::{CpInfo, CP_TAGES};

    pub fn dummy_cp() -> Vec<CpInfo> {
        vec![
            CpInfo::ConstantMethodRef {
                tag: CP_TAGES::CONSTANT_Methodref,
                class_index: 2,
                name_and_type_index: 3,
            },
            CpInfo::ConstantClassInfo {
                tag: CP_TAGES::CONSTANT_Class,
                name_index: 4,
            },
            CpInfo::ConstantNameAndType {
                tag: CP_TAGES::CONSTANT_NameAndType,
                name_index: 5,
                descriptor_index: 6,
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 16,
                bytes: "java/lang/Object".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 6,
                bytes: "<init>".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 3,
                bytes: "()V".to_owned(),
            },
            CpInfo::ConstantFieldref {
                tag: CP_TAGES::CONSTANT_Fieldref,
                class_index: 8,
                name_and_type_index: 9,
            },
            CpInfo::ConstantClassInfo {
                tag: CP_TAGES::CONSTANT_Class,
                name_index: 10,
            },
            CpInfo::ConstantNameAndType {
                tag: CP_TAGES::CONSTANT_NameAndType,
                name_index: 11,
                descriptor_index: 12,
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 16,
                bytes: "java/lang/System".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 3,
                bytes: "out".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 21,
                bytes: "Ljava/io/PrintStream;".to_owned(),
            },
            CpInfo::ConstantMethodRef {
                tag: CP_TAGES::CONSTANT_Methodref,
                class_index: 14,
                name_and_type_index: 15,
            },
            CpInfo::ConstantClassInfo {
                tag: CP_TAGES::CONSTANT_Class,
                name_index: 16,
            },
            CpInfo::ConstantNameAndType {
                tag: CP_TAGES::CONSTANT_NameAndType,
                name_index: 17,
                descriptor_index: 18,
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 9,
                bytes: "SimpleSum".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 3,
                bytes: "sum".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 3,
                bytes: "()I".to_owned(),
            },
            CpInfo::ConstantMethodRef {
                tag: CP_TAGES::CONSTANT_Methodref,
                class_index: 20,
                name_and_type_index: 21,
            },
            CpInfo::ConstantClassInfo {
                tag: CP_TAGES::CONSTANT_Class,
                name_index: 22,
            },
            CpInfo::ConstantNameAndType {
                tag: CP_TAGES::CONSTANT_NameAndType,
                name_index: 23,
                descriptor_index: 24,
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 19,
                bytes: "java/io/PrintStream".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 7,
                bytes: "println".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 4,
                bytes: "(I)V".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 4,
                bytes: "Code".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 15,
                bytes: "LineNumberTable".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 4,
                bytes: "main".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 22,
                bytes: "([Ljava/lang/String;)V".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 10,
                bytes: "SourceFile".to_owned(),
            },
            CpInfo::ConstantUtf8 {
                tag: CP_TAGES::CONSTANT_Utf8,
                length: 14,
                bytes: "SimpleSum.java".to_owned(),
            },
        ]
    }
}
