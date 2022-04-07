const ICONST_1: u8 = 0x04;
const ICONST_2: u8 = 0x05;

const ISTORE_1: u8 = 0x3C;
const ISTORE_2: u8 = 0x3d;

const ILOAD_1: u8 = 0x1B;
const ILOAD_2: u8 = 0x1C;

const IADD: u8 = 0x60;

const IRETURN: u8 = 0xAC;

fn main() {
    let code:Vec<u8> = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    let mut pc = 0;
    let mut local_variable:Vec<u8> = vec![0, 0, 0];
    let mut operand_stack:Vec<u8> = Vec::new();

    let result = loop {
        let instruction = code[pc];
        println!("DEBUG -- pc: {} instruction: {:#04x}", pc, instruction);
        match instruction {
            ICONST_1 => {
                operand_stack.push(1);
                pc += 1;
            },
            ICONST_2 => {
                operand_stack.push(2);
                pc += 1;
            },

            ISTORE_1 => {
                let val = operand_stack.pop().unwrap();
                local_variable[1] = val;
                pc += 1;
            },
            ISTORE_2 => {
                let val = operand_stack.pop().unwrap();
                local_variable[2] = val;
                pc += 1;
            }

            ILOAD_1  => {
                let val = local_variable[1];
                operand_stack.push(val);
                pc += 1;
            },
            ILOAD_2  => {
                let val = local_variable[2];
                operand_stack.push(val);
                pc += 1;
            },

            IADD => {
                let val1 = operand_stack.pop().unwrap();
                let val2 = operand_stack.pop().unwrap();

                let result = val1 + val2;
                operand_stack.push(result);
                pc += 1;
            },

            IRETURN => {
                let val = operand_stack.pop().unwrap();
                break val;
            }
            _ => break 0,
        };
    };

    println!("Hello, jvm! the result is: {}", result);
}
