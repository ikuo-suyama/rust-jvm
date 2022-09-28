use crate::binary::{read_i16, read_u16, read_u8};
use crate::instruction::Result::Return;
use crate::instruction_set::Instruction;
use crate::thread::Frame;
use std::io::Cursor;

pub enum Result {
    Return(u64),
}

pub fn instruction(frame: &mut Frame) -> Result {
    let code = &frame.current_method.get_code_attribute().code;
    let cursor = &mut Cursor::new(code.as_slice());
    let local_variable = &mut frame.local_variable;
    let operand_stack = &mut frame.operand_stack;

    loop {
        frame.pc = cursor.position();
        let instruction_code = read_u8(cursor);
        // println!(
        //     "[DEBUG] -- frame.pc: {} instruction: {:#?}(0x{:x})",
        //     frame.pc, instruction, instruction_code
        // );

        let instruction = Instruction::from(instruction_code);
        match instruction {
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

            Instruction::IF_ICMPGE => {
                let current_pc = cursor.position() - 1;
                let next_pc_offset = read_i16(cursor);
                let val2 = operand_stack.pop().unwrap();
                let val1 = operand_stack.pop().unwrap();

                if val1 >= val2 {
                    cursor.set_position((current_pc as i64 + next_pc_offset as i64) as u64);
                }
            }

            Instruction::GOTO => {
                // TODO! unsigned offset calc is suck
                let current_pc = cursor.position() - 1;
                let next_pc_offset = read_i16(cursor);
                cursor.set_position((current_pc as i64 + next_pc_offset as i64) as u64);
            }

            Instruction::IRETURN => {
                let val = operand_stack.pop().unwrap();
                break Return(val);
            }
            _ => panic!(
                "Instruction {:#?}(0x{:x}) isn't implemented yet",
                instruction, instruction_code
            ),
        }
    }
}
