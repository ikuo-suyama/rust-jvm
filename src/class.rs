use std::collections::HashMap;
use std::rc::Rc;

use crate::class_attributes::{FieldInfo, MethodInfo};
use crate::cp_info::constant_pool_value_at;

#[derive(Debug)]
pub struct MethodRef {
    pub class: String,
    pub name: String,
    pub descriptor: String,
    pub name_and_descriptor: String,
}

impl MethodRef {
    pub fn parse_from(full_descriptor: String) -> MethodRef {
        let class_and_method: Vec<&str> = full_descriptor.split(".").collect();
        let name_and_descriptor: Vec<&str> = full_descriptor.split(":").collect();

        MethodRef {
            class: String::from(class_and_method[0]),
            name: String::from(name_and_descriptor[0]),
            descriptor: String::from(name_and_descriptor[1]),
            name_and_descriptor: String::from(class_and_method[1]),
        }
    }
}

#[derive(Debug)]
pub struct ClassMeta {
    pub descriptor: String,
    pub runtime_constant_pool: Vec<String>,
    pub methods: HashMap<String, Rc<MethodInfo>>,
    pub fields: HashMap<String, Rc<FieldInfo>>,
}

impl ClassMeta {
    pub fn constant_pool_value_at(&self, index: u16) -> String {
        assert!(
            self.runtime_constant_pool.len() > index as usize,
            "constant_pool out of bounds: cp size {}, given index {}",
            self.runtime_constant_pool.len(),
            index
        );
        self.runtime_constant_pool[index as usize].clone()
    }
}
