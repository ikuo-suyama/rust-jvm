use std::collections::HashMap;
use std::rc::Rc;

use crate::class_attributes::{FieldInfo, MethodInfo};

#[derive(Debug)]
pub struct Class {
    pub descriptor: String,
    pub methods: HashMap<String, Rc<MethodInfo>>,
    pub fields: HashMap<String, Rc<FieldInfo>>,
}
