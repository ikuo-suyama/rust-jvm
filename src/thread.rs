// use std::cell::Cell;
// use std::cell::RefCell;

use crate::binary::{read_u16, read_u8};
use crate::class::Class;
use crate::class_attributes::MethodInfo;
use std::io::Cursor;

const ICONST_1: u8 = 0x04;
const ICONST_2: u8 = 0x05;

const ISTORE_0: u8 = 0x3b;
const ISTORE_1: u8 = 0x3C;
const ISTORE_2: u8 = 0x3d;

const ILOAD_0: u8 = 0x1a;
const ILOAD_1: u8 = 0x1b;
const ILOAD_2: u8 = 0x1c;

const IADD: u8 = 0x60;

const IRETURN: u8 = 0xac;

pub struct Thread<'a> {
    java_virtual_machine_stack: Vec<Frame<'a>>,
}

impl<'a> Thread<'a> {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: vec![],
        }
    }

    pub fn run(mut self, context: &'a Class, current_method: &'a MethodInfo) {
        let mut frame = Frame::create(context, current_method);

        self.java_virtual_machine_stack.push(frame);
        let top = self.java_virtual_machine_stack.len() - 1;
        let frame = self.java_virtual_machine_stack.get_mut(top).unwrap();

        frame.invoke();
    }
}

#[derive(Debug)]
pub struct Frame<'a> {
    pub pc: u64,
    pub local_variable: Vec<u64>,
    pub operand_stack: Vec<u64>,
    pub context: &'a Class,
    pub current_method: &'a MethodInfo,
}

impl<'a> Frame<'a> {
    pub fn create(context: &'a Class, current_method: &'a MethodInfo) -> Self {
        let max_locals = current_method.get_code_attribute().max_locals;
        Frame {
            pc: 0,
            local_variable: vec![0; max_locals as usize],
            operand_stack: vec![],
            context,
            current_method,
        }
    }

    pub fn invoke(&mut self) {
        let code = &self.current_method.get_code_attribute().code;
        _invoke(self, code);
    }
}

fn _invoke(frame: &mut Frame, code: &Vec<u8>) -> u64 {
    let mut cursor = &mut Cursor::new(code.as_slice());
    let mut local_variable = &mut frame.local_variable;
    let mut operand_stack = &mut frame.operand_stack;

    let result = loop {
        frame.pc = cursor.position();
        let instruction = read_u8(cursor);
        println!(
            "[DEBUG] -- frame.pc: {} instruction: {:#04x}",
            frame.pc, instruction
        );
        match instruction {
            ICONST_1 => {
                operand_stack.push(1);
            }
            ICONST_2 => {
                operand_stack.push(2);
            }

            ISTORE_0 => {
                let val = operand_stack.pop().unwrap();
                local_variable[0] = val;
            }
            ISTORE_1 => {
                let val = operand_stack.pop().unwrap();
                local_variable[1] = val;
            }
            ISTORE_2 => {
                let val = operand_stack.pop().unwrap();
                local_variable[2] = val;
            }

            ILOAD_0 => {
                let val = local_variable[0];
                operand_stack.push(val);
            }
            ILOAD_1 => {
                let val = local_variable[1];
                operand_stack.push(val);
            }
            ILOAD_2 => {
                let val = local_variable[2];
                operand_stack.push(val);
            }

            IADD => {
                let val1 = operand_stack.pop().unwrap();
                let val2 = operand_stack.pop().unwrap();

                let result = val1 + val2;
                operand_stack.push(result);
            }

            IRETURN => {
                let val = operand_stack.pop().unwrap();
                break val;
            }
            _ => panic!("Instruction 0x{:x} doesn't implement yet", instruction),
        };
    };

    println!("Hello, jvm! the result is: {}", result);
    result
}

#[test]
fn test_invoke() {
    let code: Vec<u8> = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let context = &frame_test::dummy_class();
    let current_method = &frame_test::dummy_method();
    let mut frame = Frame {
        pc: 0,
        local_variable: vec![0; 100],
        operand_stack: vec![],
        context,
        current_method,
    };

    let result = _invoke(&mut frame, &code);

    assert_eq!(result, 3);
    assert_eq!(frame.pc, 7);
    // istore_n save local value to 1
    assert_eq!(frame.local_variable[1], 1);
    assert_eq!(frame.local_variable[2], 2);
}

#[cfg(test)]
mod frame_test {
    use crate::class::Class;
    use crate::class_attributes::MethodInfo;
    use std::collections::HashMap;

    pub fn dummy_class() -> Class {
        Class {
            descriptor: "dummy".to_string(),
            methods: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    pub fn dummy_method() -> MethodInfo {
        MethodInfo {
            access_flags: 0,
            name_index: 0,
            descriptor_index: 0,
            attributes_count: 0,
            attributes: vec![],
        }
    }
}
