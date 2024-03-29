use crate::binary::read_binary_file;
use crate::class::Class;
use crate::class_file::ClassFile;
use crate::cp_info::constant_pool_value_at;
use std::collections::HashMap;

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class(&self, class_name: &String) -> Class {
        let filename = class_name.to_owned() + ".class";
        let binary = match read_binary_file(&filename) {
            Ok(class) => class,
            Err(_e) => panic!(
                "Error: Can not find or read class {}\n Reason {}",
                class_name, _e
            ),
        };

        let class_file = ClassFile::parse_from(binary.as_slice());
        // too messy, turn on when only needed...
        // println!("{:#?}", class_file);

        create_class_from(class_file)
    }
}

fn create_class_from(class_file: ClassFile) -> Class {
    let descriptor = constant_pool_value_at(&class_file.constant_pool, class_file.this_class);

    let mut constant_pool = vec![];
    // constant_pool_index start from 1. then push default to 0
    constant_pool.push(String::from(""));
    for i in 1..class_file.constant_pool_count {
        let value = constant_pool_value_at(&class_file.constant_pool, i);
        constant_pool.push(value);
    }

    let mut methods = HashMap::new();
    for method in class_file.methods {
        let method_name = constant_pool[method.name_index as usize].clone();
        let method_descriptor = constant_pool[method.descriptor_index as usize].clone();
        let method_id = format!("{}:{}", method_name, method_descriptor);
        methods.insert(method_id, method);
    }

    let mut fields = HashMap::new();
    for field in class_file.fields {
        let field_name = constant_pool[field.name_index as usize].clone();
        let field_descriptor = constant_pool[field.descriptor_index as usize].clone();
        let field_id = format!("{}:{}", field_name, field_descriptor);
        fields.insert(field_id, field);
    }

    let class = Class {
        descriptor,
        constant_pool,
        methods,
        fields,
    };
    // println!("[DEBUG] -- {:#?}", class);
    class
}

#[test]
pub fn test_create_class() {
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();
    let class_file = ClassFile::parse_from(binary.as_slice());
    let cp_count = class_file.constant_pool_count;

    let result = create_class_from(class_file);

    assert_eq!(result.descriptor, "SimpleSum");
    assert_eq!(result.constant_pool[1], "java/lang/Object.<init>:()V");
    assert_eq!(result.constant_pool.len(), cp_count as usize);

    assert!(result.methods.get("main:()I").is_some());
}
