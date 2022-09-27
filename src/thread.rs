use crate::binary::debug_bytes;
use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::interpreter::_invoke;

pub struct Thread<'a> {
    java_virtual_machine_stack: Vec<Frame<'a>>,
}

impl<'a> Thread<'a> {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: vec![],
        }
    }

    pub fn run(mut self, context: &'a Class, current_method: &'a MethodInfo) {
        let frame = Frame::create(context, current_method);

        self.java_virtual_machine_stack.push(frame);
        let top = self.java_virtual_machine_stack.len() - 1;
        let frame = self.java_virtual_machine_stack.get_mut(top).unwrap();

        frame.invoke();
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

    pub fn invoke(&mut self) {
        let code = &self.current_method.get_code_attribute().code;
        debug_bytes(code);

        _invoke(self, code);
    }
}
