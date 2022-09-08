use std::fs::File;
use std::io::{Cursor, Read};
use std::{fs, io};

use crate::java_class::ClassFile;

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class(self, _class_name: &String) -> Vec<u8> {
        let filename = _class_name.to_owned() + ".class";
        let class = match read_binary_file(filename) {
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

fn read_binary_file(filename: String) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(&filename)?;
    let metadata = fs::metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    return Ok(buffer);
}
