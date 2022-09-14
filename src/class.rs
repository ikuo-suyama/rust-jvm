use crate::class_attributes::{FieldInfo, MethodInfo};
use crate::class_file::ClassFile;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Class {
    pub descriptor: String,
    pub methods: HashMap<String, MethodInfo>,
    pub fields: HashMap<String, FieldInfo>,
}
