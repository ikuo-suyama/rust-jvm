// use std::cell::Cell;
// use std::cell::RefCell;

use crate::class::Class;
use crate::class_attributes::MethodInfo;

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
    pub pc: u32,
    pub local_variable: Vec<u64>,
    pub operand_stack: Vec<u64>,
    pub context: &'a Class,
    pub current_method: &'a MethodInfo,
}

impl<'a> Frame<'a> {
    pub fn create(context: &'a Class, current_method: &'a MethodInfo) -> Self {
        Frame {
            pc: 0,
            local_variable: vec![],
            operand_stack: vec![],
            context,
            current_method,
        }
    }

    pub fn invoke(&self) -> u8 {
        let code = &self.current_method.get_code_attribute().code;
        self._invoke(code)
    }
    fn _invoke(&self, code: &Vec<u8>) -> u8 {
        let mut pc = 0;
        let mut local_variable: Vec<u8> = vec![0, 0, 0];
        let mut operand_stack: Vec<u8> = Vec::new();

        let result = loop {
            let instruction = code[pc];
            println!("[DEBUG] -- pc: {} instruction: {:#04x}", pc, instruction);
            match instruction {
                ICONST_1 => {
                    operand_stack.push(1);
                    pc += 1;
                }
                ICONST_2 => {
                    operand_stack.push(2);
                    pc += 1;
                }

                ISTORE_0 => {
                    let val = operand_stack.pop().unwrap();
                    local_variable[0] = val;
                    pc += 1;
                }
                ISTORE_1 => {
                    let val = operand_stack.pop().unwrap();
                    local_variable[1] = val;
                    pc += 1;
                }
                ISTORE_2 => {
                    let val = operand_stack.pop().unwrap();
                    local_variable[2] = val;
                    pc += 1;
                }

                ILOAD_0 => {
                    let val = local_variable[0];
                    operand_stack.push(val);
                    pc += 1;
                }
                ILOAD_1 => {
                    let val = local_variable[1];
                    operand_stack.push(val);
                    pc += 1;
                }
                ILOAD_2 => {
                    let val = local_variable[2];
                    operand_stack.push(val);
                    pc += 1;
                }

                IADD => {
                    let val1 = operand_stack.pop().unwrap();
                    let val2 = operand_stack.pop().unwrap();

                    let result = val1 + val2;
                    operand_stack.push(result);
                    pc += 1;
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
}

#[test]
fn test_invoke() {
    let code: Vec<u8> = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let class = frame_test::dummy_class();
    let method = frame_test::dummy_method();
    let frame = Frame::create(&class, &method);

    let result = frame._invoke(&code);

    assert_eq!(result, 3);
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
