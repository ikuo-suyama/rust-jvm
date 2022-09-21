use crate::binary::debug_bytes;
use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::interpreter::interpret;
use crate::JVM;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Thread {
    pub java_virtual_machine_stack: RefCell<Vec<Frame>>,
}

impl Thread {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: RefCell::new(vec![]),
        }
    }

    pub fn push_frame(&self, frame: Frame) {
        let mut frame_stack = self.java_virtual_machine_stack.borrow_mut();
        frame_stack.push(frame);
    }

    pub fn pop_frame(&self) -> Frame {
        let mut frame_stack = self.java_virtual_machine_stack.borrow_mut();
        frame_stack.pop().unwrap()
    }

    pub fn get_current_frame(&self) -> Rc<Frame> {
        let current_frame = self.pop_frame();
        let frame_ref = Rc::new(current_frame);
        self.push_frame(current_frame);
        frame_ref
    }
}

#[derive(Debug)]
pub struct Frame {
    pub pc: RefCell<u64>,
    pub local_variable: RefCell<Vec<u64>>,
    pub operand_stack: RefCell<Vec<u64>>,
    pub context: Rc<Class>,
    pub current_method: Rc<MethodInfo>,
}

impl Frame {
    pub fn create(context: &Rc<Class>, current_method: &Rc<MethodInfo>) -> Self {
        let max_locals = current_method.get_code_attribute().max_locals;
        Frame {
            pc: RefCell::new(0),
            local_variable: RefCell::new(vec![0; max_locals as usize]),
            operand_stack: RefCell::new(vec![]),
            context: Rc::clone(context),
            current_method: Rc::clone(current_method),
        }
    }
}
