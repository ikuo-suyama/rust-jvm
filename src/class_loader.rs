use crate::binary::read_binary_file;

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class(self, _class_name: &String) -> Vec<u8> {
        let filename = _class_name.to_owned() + ".class";
        let class = match read_binary_file(&filename) {
            Ok(class) => class,
            Err(_e) => panic!(
                "Error: Can not find or read class {}\n Reason {}",
                _class_name, _e
            ),
        };
        println!("[DEBUG] -- {:x?}", class);

        return vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    }
}
