use crate::binary::read_binary_file;
use crate::java_class::Class;
use crate::java_class::ClassFile;
use std::collections::HashMap;

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class<'a>(
        self,
        native_area: &'a mut HashMap<String, Class>,
        class_name: &String,
    ) -> &'a Class {
        let filename = class_name.to_owned() + ".class";
        let binary = match read_binary_file(&filename) {
            Ok(class) => class,
            Err(_e) => panic!(
                "Error: Can not find or read class {}\n Reason {}",
                class_name, _e
            ),
        };

        let class_file = ClassFile::parse_from(binary.as_slice());
        let class = Class::createFrom(class_file);
        native_area.insert("test_key".to_owned(), class);
        native_area.get("test_key").unwrap()
    }
}
