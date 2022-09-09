/* using trait
pub trait CpInfo {
    fn tag(&self) -> u8;
}

struct ConstantClassInfo {
    pub tag: u8,
    pub name_index: u16,
}

impl CpInfo for ConstantClassInfo {
    fn tag(&self) -> u8 {
        self.tag
    }
}

fn parse_cp_info(cursor: &mut Cursor<&[u8]>) -> Vec<Box<dyn CpInfo>> {
    /// here we need a [Box](https://doc.rust-jp.rs/rust-by-example-ja/std/box.html)
    /// to avoid "the trait `Sized` is not implemented for..."
    vec![Box::new(ConstantClassInfo {
        tag: 0,
        name_index: 0,
    })]
}
*/

use crate::binary::{read_binary_file, read_u16, read_u32};
use crate::cp_info::CpInfo::ConstantClassInfo;
use std::io::Cursor;

pub enum CpInfo {
    ConstantClassInfo { tag: u8, name_index: u16 },
}

pub fn parse_cp_info(cursor: &mut Cursor<&[u8]>, constant_pool_count: u16) -> Vec<CpInfo> {
    let mut cp_info: Vec<CpInfo> = vec![];

    for _ in 0..constant_pool_count {
        cp_info.push(ConstantClassInfo {
            tag: 0,
            name_index: 0,
        })
    }

    cp_info
}

#[test]
fn test_parse_cp_info() {
    let binary = read_binary_file(&"java/SimpleSum.class".to_owned()).unwrap();
    let mut cursor = Cursor::new(binary.as_slice());

    // skip consts
    let _ = read_u32(&mut cursor);
    let _ = read_u16(&mut cursor);
    let _ = read_u16(&mut cursor);
    let constant_pool_count = read_u16(&mut cursor);

    let result = parse_cp_info(&mut cursor, constant_pool_count);
    assert_eq!(result.len(), constant_pool_count as usize);
}
