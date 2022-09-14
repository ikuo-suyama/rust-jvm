use crate::binary::read_binary_file;
use crate::java_class::ClassFile;

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class(self, _class_name: &String) -> ClassFile {
        let filename = _class_name.to_owned() + ".class";
        let binary = match read_binary_file(&filename) {
            Ok(class) => class,
            Err(_e) => panic!(
                "Error: Can not find or read class {}\n Reason {}",
                _class_name, _e
            ),
        };
        ClassFile::parse_from(binary.as_slice())
    }
}
