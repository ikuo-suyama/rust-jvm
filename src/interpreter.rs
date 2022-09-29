use crate::instruction::instruction;
use crate::instruction::Invokes::InvokeStatic;
use crate::instruction::Result::{Invoke, Return};
use crate::instruction_set::Instruction;
use crate::invoke::{i_return, invoke_static};
use crate::thread::{Frame, Thread};
use crate::JVM;

pub fn interpret(thread: &mut Thread) {
    let mut counter = 0;
    while thread.java_virtual_machine_stack.len() >= 1 {
        counter = counter + 1;
        if counter > 10 {
            panic!()
        }

        let frame = thread.java_virtual_machine_stack.last_mut().unwrap();

        match instruction(frame) {
            Return(v) => i_return(thread, v),
            Invoke(invoke) => {
                if let InvokeStatic { cp_index } = invoke {
                    invoke_static(thread, cp_index)
                }
            }
        };
    }
}
