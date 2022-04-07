const ICONST_1: u8 = 0x04;
const ISTORE_1: u8 = 0x3C;

fn main() {
    let code:Vec<u8> = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60];
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
            ISTORE_1 => {
                let val = operand_stack.pop();
                local_variable[1] = val.unwrap();
                pc += 1;
            }
            _ => break 0,
        };
    };

    println!("Hello, jvm! the result is: {}", result);
}
