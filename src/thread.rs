use crate::binary::debug_bytes;
use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::interpreter::interpret;
use crate::JVM;

pub struct Thread<'a> {
    java_virtual_machine_stack: Vec<Frame<'a>>,
}

impl<'a> Thread<'a> {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: vec![],
        }
    }

    pub fn run(&'a mut self, context: &'a Class, current_method: &'a MethodInfo) {
        let frame = Frame::create(context, current_method);

        self.push_frame(frame);
        let frame = self.get_current_frame();

        interpret(frame);
    }

    pub fn push_frame<'b>(&'b mut self, frame: Frame<'a>) {
        self.java_virtual_machine_stack.push(frame);
    }

    pub fn get_current_frame(&'a mut self) -> &'a mut Frame<'a> {
        let top = self.java_virtual_machine_stack.len() - 1;
        let maybe_current_frame = self.java_virtual_machine_stack.get_mut(top);
        maybe_current_frame.unwrap()
    }
}

#[derive(Debug)]
pub struct Frame<'a> {
    pub pc: u64,
    pub local_variable: Vec<u64>,
    pub operand_stack: Vec<u64>,
    pub context: &'a Class,
    pub current_method: &'a MethodInfo,
}

impl<'a> Frame<'a> {
    pub fn create(context: &'a Class, current_method: &'a MethodInfo) -> Self {
        let max_locals = current_method.get_code_attribute().max_locals;
        Frame {
            pc: 0,
            local_variable: vec![0; max_locals as usize],
            operand_stack: vec![],
            context,
            current_method,
        }
    }
}
