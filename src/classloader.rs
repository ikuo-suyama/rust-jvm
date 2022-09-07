use std::fs::File;
use std::io::Read;
use std::{fs, io};

pub struct ClassLoader {}

impl ClassLoader {
    pub fn load_class(self, _class_name: &String) -> Vec<u8> {
        let class = match read_binary(_class_name) {
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

fn read_binary(filename: &String) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(&filename)?;
    let metadata = fs::metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    return Ok(buffer);
}
