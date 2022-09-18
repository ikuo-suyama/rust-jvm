use crate::binary::read_u8;
use crate::instruction_set::Instruction;
use crate::thread::Frame;
use std::io::Cursor;

// TODO: 1. sprit instruction code to function
// TODO: 2. create return type for instruction, e.g. CONTINUE/RETURN/INVOKE
pub fn _invoke(frame: &mut Frame, code: &Vec<u8>) -> u64 {
    let cursor = &mut Cursor::new(code.as_slice());
    let local_variable = &mut frame.local_variable;
    let operand_stack = &mut frame.operand_stack;

    let result = loop {
        frame.pc = cursor.position();
        let instruction_code = read_u8(cursor);
        let instruction = Instruction::from(instruction_code);
        println!(
            "[DEBUG] -- frame.pc: {} instruction: {:#?}(0x{:x})",
            frame.pc, instruction, instruction_code
        );
        match instruction {
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
                let val1 = operand_stack.pop().unwrap();
                let val2 = operand_stack.pop().unwrap();

                let result = val1 + val2;
                operand_stack.push(result);
            }

            Instruction::IRETURN => {
                let val = operand_stack.pop().unwrap();
                break val;
            }
            _ => panic!(
                "Instruction {:#?}(0x{:x}) doesn't implement yet",
                instruction, instruction_code
            ),
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
