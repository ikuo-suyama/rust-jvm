use std::collections::HashMap;
use std::rc::Rc;

use crate::class_attributes::{FieldInfo, MethodInfo};
use crate::cp_info::constant_pool_value_at;

#[derive(Debug)]
pub struct Class {
    pub descriptor: String,
    pub constant_pool: Vec<String>,
    pub methods: HashMap<String, Rc<MethodInfo>>,
    pub fields: HashMap<String, Rc<FieldInfo>>,
}

impl Class {
    pub fn constant_pool_value_at(&self, index: u16) -> String {
        assert!(
            self.constant_pool.len() > index as usize,
            "constant_pool out of bounds: cp size {}, given index {}",
            self.constant_pool.len(),
            index
        );
        self.constant_pool[index as usize].clone()
    }
}
