use crate::binary::read_binary_file;
use crate::class::Class;
use crate::class_file::ClassFile;
use std::collections::HashMap;

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class<'a>(
        self,
        method_area: &'a mut HashMap<String, Class>,
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
        // too messy, turn on when only needed...
        // println!("{:#?}", class_file);

        let class = create_class_from(class_file);
        register_method_area(method_area, class)
    }
}

fn register_method_area(method_area: &mut HashMap<String, Class>, class: Class) -> &Class {
    let descriptor = class.descriptor.clone();
    method_area.insert(descriptor.clone(), class);
    method_area.get(&*descriptor).unwrap()
}

fn create_class_from(class_file: ClassFile) -> Class {
    Class {
        descriptor: "".to_string(),
        methods: Default::default(),
        fields: Default::default(),
    }
}
