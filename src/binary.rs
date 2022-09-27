use std::fs::File;
use std::io::{Cursor, Read};
use std::{fs, io};

pub fn read_binary_file(filename: &String) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(&filename)?;
    let metadata = fs::metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    return Ok(buffer);
}

pub fn read_i8(cursor: &mut Cursor<&[u8]>) -> i8 {
    read_u8(cursor) as i8
}

pub fn read_i16(cursor: &mut Cursor<&[u8]>) -> i16 {
    read_u16(cursor) as i16
}

pub fn read_u8(cursor: &mut Cursor<&[u8]>) -> u8 {
    let buf: &mut [u8] = &mut [0; 1];
    cursor.read_exact(buf).unwrap();
    buf[0]
}

pub fn read_u16(cursor: &mut Cursor<&[u8]>) -> u16 {
    let buf: &mut [u8] = &mut [0; 2];
    cursor.read_exact(buf).unwrap();
    let left: u16 = buf[0] as u16;
    let right: u16 = buf[1] as u16;
    (left << 8) + right
}

pub fn read_u32(cursor: &mut Cursor<&[u8]>) -> u32 {
    let buf: &mut [u8] = &mut [0; 4];
    cursor.read_exact(buf).unwrap();
    let mut result: u32 = buf[0] as u32;
    let mut itr = 0;
    loop {
        result = result << 8;
        result += buf[itr] as u32;
        itr += 1;
        if itr >= 4 {
            break;
        }
    }
    result
}

pub fn read_string_to(cursor: &mut Cursor<&[u8]>, length: usize) -> String {
    let buf: &mut [u8] = &mut vec![0; length];
    cursor.read_exact(buf).unwrap();
    std::str::from_utf8(buf).unwrap().to_string()
}

pub fn read_to(cursor: &mut Cursor<&[u8]>, length: usize) -> Vec<u8> {
    let buf: &mut [u8] = &mut vec![0; length];
    cursor.read_exact(buf).unwrap();
    buf.to_owned()
}

pub fn debug_bytes(bytes: &Vec<u8>) {
    let s: String = bytes
        .iter()
        .map(|byte| format!("0x{:02x}, ", byte))
        .collect();
    println!("[DEBUG] -- [{}]", s);
}

#[test]
fn test_read_i8() {
    // https://towardsdatascience.com/unsinged-signed-integers-and-casting-in-rust-9a847bfc398f
    let bytes: &[u8] = &[0b10000000];
    println!("{:b}", bytes[0]);
    let mut cursor = Cursor::new(bytes);

    let result = read_i8(&mut cursor);

    assert_eq!(result, -128);
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

#[test]
fn test_read_u32() {
    let bytes: &[u8] = &[0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let mut cursor = Cursor::new(bytes);

    let _ = read_u8(&mut cursor);
    let _ = read_u16(&mut cursor);
    let result = read_u32(&mut cursor);

    assert_eq!(result, 0x3D1B1C60_u32);
    assert_eq!(cursor.position(), 7);
}

#[test]
fn test_read_string_to() {
    let bytes: &[u8] = &[
        0x6A, 0x61, 0x76, 0x61, 0x2F, 0x6C, 0x61, 0x6E, 0x67, 0x2F, 0x4F, 0x62, 0x6A, 0x65, 0x63,
        0x74,
    ];

    let mut cursor = Cursor::new(bytes);

    let result = read_string_to(&mut cursor, bytes.len());

    assert_eq!(result, "java/lang/Object");
}

#[test]
fn test_read_to() {
    let bytes: &[u8] = &[
        0x6A, 0x61, 0x76, 0x61, 0x2F, 0x6C, 0x61, 0x6E, 0x67, 0x2F, 0x4F, 0x62, 0x6A, 0x65, 0x63,
        0x74,
    ];

    let mut cursor = Cursor::new(bytes);

    let result = read_to(&mut cursor, bytes.len());

    assert_eq!(result, bytes);
}
