use std::collections::HashMap;

use crate::class_attributes::{FieldInfo, MethodInfo};

#[derive(Debug)]
pub struct Class {
    pub descriptor: String,
    pub methods: HashMap<String, MethodInfo>,
    pub fields: HashMap<String, FieldInfo>,
}
