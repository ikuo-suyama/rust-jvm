use crate::class_attributes::{FieldInfo, MethodInfo};
use crate::java_class::ClassFile;
use std::collections::HashMap;

pub struct Class {
    pub methods: HashMap<String, MethodInfo>,
    pub fields: HashMap<String, FieldInfo>,
}

impl Class {
    pub(crate) fn createFrom(class_file: ClassFile) -> Class {
        Class {
            methods: HashMap::new(),
            fields: HashMap::new(),
        }
    }
}
