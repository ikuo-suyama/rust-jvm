use std::fs::read;
use std::io::Cursor;
use std::rc::Rc;

use crate::binary::{read_i16, read_u16, read_u8};
use crate::instruction::instruction;
use crate::instruction::Invokes::InvokeStatic;
use crate::instruction_set::Instruction;
use crate::thread::{Frame, Thread};

use crate::instruction::Result::{Invoke, Return};
use crate::invoke::{i_return, invoke_static};
use crate::JVM;

pub fn interpret(thread: &mut Thread) {
    while thread.java_virtual_machine_stack.len() >= 1 {
        let frame = thread.java_virtual_machine_stack.last_mut().unwrap();

        match instruction(frame) {
            Return(v) => {
                println!("Hello, jvm! the result is: {}", v);
                i_return(thread, v)
            }
            Invoke(invoke) => {
                if let InvokeStatic { cp_index } = invoke {
                    invoke_static(thread, cp_index)
                }
            }
        };
    }
}
