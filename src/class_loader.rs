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

fn read_u8(cursor: &mut Cursor<&[u8]>) -> u8 {
    let buf: &mut [u8] = &mut [0; 1];
    cursor.read_exact(buf).unwrap();
    buf[0]
}

fn read_u16(cursor: &mut Cursor<&[u8]>) -> u16 {
    let buf: &mut [u8] = &mut [0; 2];
    cursor.read_exact(buf).unwrap();
    let left: u16 = buf[0] as u16;
    let right: u16 = buf[1] as u16;
    (left << 8) + right
}

#[test]
fn test_read_u8() {
    let bytes: &[u8] = &[0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let mut cursor = Cursor::new(bytes);

    let result = read_u8(&mut cursor);

    assert_eq!(result, 0x04);
    assert_eq!(cursor.position(), 1);
}

#[test]
fn test_read_u16() {
    let bytes: &[u8] = &[0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let mut cursor = Cursor::new(bytes);

    let _ = read_u8(&mut cursor);
    let result = read_u16(&mut cursor);

    assert_eq!(result, 0x3C05_u16);
    assert_eq!(cursor.position(), 3);
}
