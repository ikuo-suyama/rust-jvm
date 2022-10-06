use crate::instruction::Invokes::InvokeStatic;
use crate::instruction::Result::{Invoke, Return};
use crate::instruction::{instruction, Returns};
use crate::invoke::{i_return, invoke_static, java_return};
use crate::thread::Thread;

const MAX_LOOP_COUNT: i32 = 1_000_000;

pub fn interpret(thread: &mut Thread) {
    let mut counter = 0;
    while thread.java_virtual_machine_stack.len() >= 1 {
        if counter > MAX_LOOP_COUNT {
            panic!("[ERROR] MAX_LOOP_COUNT exceeded: Infinite loops are suspected.")
        }
        counter = counter + 1;

        let frame = thread.java_virtual_machine_stack.last_mut().unwrap();

        match instruction(frame) {
            Return(ret) => match ret {
                Returns::IReturn { val } => i_return(thread, val),
                Returns::Return => java_return(thread),
            },
            Invoke(invoke) => {
                if let InvokeStatic { cp_index } = invoke {
                    invoke_static(thread, cp_index)
                }
            }
        };
    }
}
