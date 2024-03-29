use crate::binary::{debug_bytes, read_i16, read_u16, read_u8};
use crate::instruction::Invokes::InvokeStatic;
use crate::instruction::Result::{Invoke, Return};
use crate::instruction::Returns::IReturn;
use crate::instruction_set::Instruction;
use crate::thread::Frame;
use std::io;
use std::io::{Cursor, Write};

#[derive(Debug)]
pub enum Invokes {
    InvokeStatic { cp_index: u16 },
}

#[derive(Debug)]
pub enum Returns {
    IReturn { val: u64 },
    Return,
}

#[derive(Debug)]
pub enum Result {
    Invoke(Invokes),
    Return(Returns),
}

pub fn instruction(frame: &mut Frame) -> Result {
    let code = &frame.current_method.get_code_attribute().code;
    // debug_bytes(code);

    let cursor = &mut Cursor::new(code.as_slice());
    cursor.set_position(frame.pc);

    let local_variable = &mut frame.local_variable;
    let operand_stack = &mut frame.operand_stack;

    let result = loop {
        frame.pc = cursor.position();
        let instruction_code = read_u8(cursor);
        let instruction = Instruction::from(instruction_code);
        // println!(
        //     "[VERBOSE] -- frame.pc: {} instruction: {:#?}(0x{:x})",
        //     frame.pc, instruction, instruction_code
        // );

        match instruction {
            Instruction::BIPUSH => {
                // TODO: handle type...
                let val = read_u8(cursor);
                operand_stack.push(val as u64);
            }

            Instruction::ICONST_0 => {
                operand_stack.push(0);
            }
            Instruction::ICONST_1 => {
                operand_stack.push(1);
            }
            Instruction::ICONST_2 => {
                operand_stack.push(2);
            }

            Instruction::ISTORE_0 => {
                let val = operand_stack.pop().unwrap();
                local_variable[0] = val;
            }
            Instruction::ISTORE_1 => {
                let val = operand_stack.pop().unwrap();
                local_variable[1] = val;
            }
            Instruction::ISTORE_2 => {
                let val = operand_stack.pop().unwrap();
                local_variable[2] = val;
            }

            Instruction::ILOAD_0 => {
                let val = local_variable[0];
                operand_stack.push(val);
            }
            Instruction::ILOAD_1 => {
                let val = local_variable[1];
                operand_stack.push(val);
            }
            Instruction::ILOAD_2 => {
                let val = local_variable[2];
                operand_stack.push(val);
            }

            Instruction::IADD => {
                let val2 = operand_stack.pop().unwrap();
                let val1 = operand_stack.pop().unwrap();

                let result = val1 + val2;
                operand_stack.push(result);
            }

            Instruction::ISUB => {
                let val2 = operand_stack.pop().unwrap();
                let val1 = operand_stack.pop().unwrap();

                let result = val1 - val2;
                operand_stack.push(result);
            }

            Instruction::SIPUSH => {
                // TODO: handle type...
                let val = read_u16(cursor);
                operand_stack.push(val as u64);
            }

            Instruction::IINC => {
                let index = read_u8(cursor);
                let const_val = read_u8(cursor);

                local_variable[index as usize] += const_val as u64;
            }

            Instruction::IFGT => {
                let next_pc_offset = read_i16(cursor);
                let val = operand_stack.pop().unwrap();

                if val > 0 {
                    goto_offset(cursor, frame.pc, next_pc_offset);
                }
            }

            Instruction::IF_ICMPGE => {
                // pc offset should be read anyway, regardless the comp result
                let next_pc_offset = read_i16(cursor);
                let val2 = operand_stack.pop().unwrap();
                let val1 = operand_stack.pop().unwrap();

                if val1 >= val2 {
                    goto_offset(cursor, frame.pc, next_pc_offset);
                }
            }

            Instruction::IF_ICMPNE => {
                // pc offset should be read anyway, regardless the comp result
                let next_pc_offset = read_i16(cursor);
                let val2 = operand_stack.pop().unwrap();
                let val1 = operand_stack.pop().unwrap();

                if val1 != val2 {
                    goto_offset(cursor, frame.pc, next_pc_offset);
                }
            }

            Instruction::GOTO => {
                let next_pc_offset = read_i16(cursor);
                goto_offset(cursor, frame.pc, next_pc_offset);
            }

            Instruction::POP => {
                operand_stack.pop();
            }

            /// invoke
            Instruction::INVOKESTATIC => {
                let cp_index = read_u16(cursor);
                break Invoke(InvokeStatic { cp_index });
            }

            /// return
            Instruction::IRETURN => {
                let val = operand_stack.pop().unwrap();
                break Return(IReturn { val });
            }

            Instruction::RETURN => {
                break Return(Returns::Return);
            }

            _ => panic!(
                "Instruction {:#?}(0x{:x}) isn't implemented yet",
                instruction, instruction_code
            ),
        }
    };
    frame.pc = cursor.position();
    result
}

fn goto_offset(cursor: &mut Cursor<&[u8]>, current_pc: u64, offset: i16) {
    // TODO! unsigned offset calc is suck
    cursor.set_position((current_pc as i64 + offset as i64) as u64);
}

#[test]
fn test_invoke_sum() {
    use std::rc::Rc;

    let code: Vec<u8> = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let context = frame_test::dummy_class();
    let mut current_method = frame_test::dummy_method(code);
    let mut frame = Frame::create(&Rc::new(context), &Rc::new(current_method));

    let result = instruction(&mut frame);

    assert!(matches!(result, Return(IReturn { val: 3 })));
}

#[test]
fn test_invoke_loop() {
    use std::rc::Rc;

    let code: Vec<u8> = vec![
        0x03, 0x3b, 0x03, 0x3c, 0x1b, 0x11, 0x27, 0x10, 0xa2, 0x00, 0x0d, 0x1a, 0x1b, 0x60, 0x3b,
        0x84, 0x01, 0x01, 0xa7, 0xff, 0xf2, 0x1a, 0xac,
    ];
    let context = frame_test::dummy_class();
    let current_method = frame_test::dummy_method(code);
    let mut frame = Frame::create(&Rc::new(context), &Rc::new(current_method));

    let result = instruction(&mut frame);

    assert!(matches!(result, Return(IReturn { val: 49995000 })));
}

#[test]
fn test_invoke_static() {
    use std::rc::Rc;

    // code from Fibonacci.fib
    let code: Vec<u8> = vec![
        0x1a, 0x9d, 0x00, 0x05, 0x03, 0xac, 0x1a, 0x04, 0xa0, 0x00, 0x05, 0x04, 0xac, 0x1a, 0x04,
        0x64, 0xb8, 0x00, 0x07, 0x1a, 0x05, 0x64, 0xb8, 0x00, 0x07, 0x60, 0xac,
    ];
    let context = frame_test::dummy_class();
    let current_method = frame_test::dummy_method(code);
    let mut frame = Frame::create(&Rc::new(context), &Rc::new(current_method));
    // argument i = 2
    frame.local_variable[0] = 2;

    let result = instruction(&mut frame);

    assert!(matches!(result, Invoke(InvokeStatic { cp_index: 7 })))
}

#[cfg(test)]
pub mod frame_test {
    use std::collections::HashMap;

    use crate::class::Class;
    use crate::class_attributes::{AttributeInfo, CodeAttributeInfo, MethodInfo};

    pub fn dummy_class() -> Class {
        Class {
            descriptor: "dummy".to_string(),
            constant_pool: vec![],
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
