use std::fs::read;
use std::io::Cursor;
use std::rc::Rc;

use crate::binary::{read_i16, read_u16, read_u8};
use crate::instruction::instruction;
use crate::instruction_set::Instruction;
use crate::thread::{Frame, Thread};

use crate::instruction::Result::Return;
use crate::JVM;

pub fn interpret(thread: &mut Thread) -> u64 {
    let mut result = 0;

    while thread.java_virtual_machine_stack.len() >= 1 {
        let frame = thread.java_virtual_machine_stack.last_mut().unwrap();

        result = match instruction(frame) {
            Return(v) => v,
            _ => 0,
        };

        // TODO:
        thread.java_virtual_machine_stack.pop();
    }

    println!("Hello, jvm! the result is: {}", result);
    result
}
