use std::fs::read;
use std::io::Cursor;
use std::rc::Rc;

use crate::binary::{read_i16, read_u16, read_u8};
use crate::instruction::instruction;
use crate::instruction_set::Instruction;
use crate::thread::Frame;

use crate::instruction::Result::Return;
use crate::JVM;

pub fn interpret(frame: &mut Frame) -> u64 {
    let code = &frame.current_method.get_code_attribute().code;
    let cursor = &mut Cursor::new(code.as_slice());
    let local_variable = &mut frame.local_variable;
    let operand_stack = &mut frame.operand_stack;

    let result = loop {
        frame.pc = cursor.position();
        let instruction_code = read_u8(cursor);
        // println!(
        //     "[DEBUG] -- frame.pc: {} instruction: {:#?}(0x{:x})",
        //     frame.pc, instruction, instruction_code
        // );
        match instruction(cursor, instruction_code, local_variable, operand_stack) {
            Return(v) => break v,
            _ => {}
        }
    };

    println!("Hello, jvm! the result is: {}", result);
    result
}

#[test]
fn test_invoke_sum() {
    let code: Vec<u8> = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let context = frame_test::dummy_class();
    let mut current_method = frame_test::dummy_method(code);
    let mut frame = Frame::create(&Rc::new(context), &Rc::new(current_method));

    let result = interpret(&mut frame);

    assert_eq!(result, 3);
    assert_eq!(frame.pc, 7);
    // istore_n save local value to 1
    assert_eq!(frame.local_variable[1], 1);
    assert_eq!(frame.local_variable[2], 2);
}

#[test]
fn test_invoke_loop() {
    let code: Vec<u8> = vec![
        0x03, 0x3b, 0x03, 0x3c, 0x1b, 0x11, 0x27, 0x10, 0xa2, 0x00, 0x0d, 0x1a, 0x1b, 0x60, 0x3b,
        0x84, 0x01, 0x01, 0xa7, 0xff, 0xf2, 0x1a, 0xac,
    ];
    let context = frame_test::dummy_class();
    let current_method = frame_test::dummy_method(code);
    let mut frame = Frame::create(&Rc::new(context), &Rc::new(current_method));

    let result = interpret(&mut frame);

    assert_eq!(result, 49995000)
}

#[cfg(test)]
pub mod frame_test {
    use std::collections::HashMap;

    use crate::class::Class;
    use crate::class_attributes::{AttributeInfo, CodeAttributeInfo, MethodInfo};

    pub fn dummy_class() -> Class {
        Class {
            descriptor: "dummy".to_string(),
            methods: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    pub fn dummy_method(code: Vec<u8>) -> MethodInfo {
        MethodInfo {
            access_flags: 0,
            name_index: 0,
            descriptor_index: 0,
            attributes_count: 1,
            attributes: vec![dummy_code(code)],
        }
    }

    pub fn dummy_code(code: Vec<u8>) -> AttributeInfo {
        AttributeInfo::CodeAttributeInfo(CodeAttributeInfo {
            attribute_name_index: 0,
            attribute_length: 0,
            max_stack: 100,
            max_locals: 100,
            code_length: 0,
            code,
            exception_table_length: 0,
            exception_table: vec![],
            attributes_count: 0,
            attributes: vec![],
        })
    }
}
