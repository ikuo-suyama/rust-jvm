use crate::binary::{read_i16, read_u16, read_u8};
use crate::instruction::Result::Continue;
use crate::instruction::Result::Return;
use crate::instruction_set::Instruction;
use std::io::Cursor;

pub enum Result {
    Return(u64),
    Continue,
}

pub fn instruction(
    cursor: &mut Cursor<&[u8]>,
    instruction_code: u8,
    local_variable: &mut Vec<u64>,
    operand_stack: &mut Vec<u64>,
) -> Result {
    let instruction = Instruction::from(instruction_code);
    match instruction {
        Instruction::ICONST_0 => {
            operand_stack.push(0);
            Continue
        }
        Instruction::ICONST_1 => {
            operand_stack.push(1);
            Continue
        }
        Instruction::ICONST_2 => {
            operand_stack.push(2);
            Continue
        }

        Instruction::ISTORE_0 => {
            let val = operand_stack.pop().unwrap();
            local_variable[0] = val;
            Continue
        }
        Instruction::ISTORE_1 => {
            let val = operand_stack.pop().unwrap();
            local_variable[1] = val;
            Continue
        }
        Instruction::ISTORE_2 => {
            let val = operand_stack.pop().unwrap();
            local_variable[2] = val;
            Continue
        }

        Instruction::ILOAD_0 => {
            let val = local_variable[0];
            operand_stack.push(val);
            Continue
        }
        Instruction::ILOAD_1 => {
            let val = local_variable[1];
            operand_stack.push(val);
            Continue
        }
        Instruction::ILOAD_2 => {
            let val = local_variable[2];
            operand_stack.push(val);
            Continue
        }

        Instruction::IADD => {
            let val2 = operand_stack.pop().unwrap();
            let val1 = operand_stack.pop().unwrap();

            let result = val1 + val2;
            operand_stack.push(result);
            Continue
        }

        Instruction::SIPUSH => {
            // TODO: handle type...
            let val = read_u16(cursor);
            operand_stack.push(val as u64);
            Continue
        }

        Instruction::IINC => {
            let index = read_u8(cursor);
            let const_val = read_u8(cursor);

            local_variable[index as usize] += const_val as u64;
            Continue
        }

        Instruction::IF_ICMPGE => {
            let current_pc = cursor.position() - 1;
            let next_pc_offset = read_i16(cursor);
            let val2 = operand_stack.pop().unwrap();
            let val1 = operand_stack.pop().unwrap();

            if val1 >= val2 {
                cursor.set_position((current_pc as i64 + next_pc_offset as i64) as u64);
            }
            Continue
        }

        Instruction::GOTO => {
            // TODO! unsigned offset calc is suck
            let current_pc = cursor.position() - 1;
            let next_pc_offset = read_i16(cursor);
            cursor.set_position((current_pc as i64 + next_pc_offset as i64) as u64);
            Continue
        }

        Instruction::IRETURN => {
            let val = operand_stack.pop().unwrap();
            Return(val)
        }
        _ => panic!(
            "Instruction {:#?}(0x{:x}) isn't implemented yet",
            instruction, instruction_code
        ),
    }
}
