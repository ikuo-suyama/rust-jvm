use crate::binary::debug_bytes;
use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::interpreter::interpret;
use crate::JVM;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Thread {
    /// If we want to push current frame to the stack,
    /// It means we need multiple-ref and mut.
    /// Then use Rc<RefCell<T>
    pub java_virtual_machine_stack: Vec<Rc<RefCell<Frame>>>,
}

impl Thread {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: vec![],
        }
    }
}

pub trait JavaVirtualMachineStack {
    fn push_frame(&mut self, frame: Frame);
    fn pop_frame(&mut self) -> Rc<RefCell<Frame>>;
    fn get_current_frame(&self) -> Rc<RefCell<Frame>>;
}

impl JavaVirtualMachineStack for Thread {
    fn push_frame(&mut self, frame: Frame) {
        let frame_ref = Rc::new(RefCell::new(frame));
        self.java_virtual_machine_stack.push(frame_ref);
    }

    fn pop_frame(&mut self) -> Rc<RefCell<Frame>> {
        self.java_virtual_machine_stack.pop().unwrap()
    }

    fn get_current_frame(&self) -> Rc<RefCell<Frame>> {
        let frame_ref = self.java_virtual_machine_stack.last().unwrap();

        Rc::clone(frame_ref)
    }
}

#[derive(Debug)]
pub struct Frame {
    pub pc: u64,
    pub local_variable: Vec<u64>,
    pub operand_stack: Vec<u64>,
    pub context: Rc<Class>,
    pub current_method: Rc<MethodInfo>,
}

impl Frame {
    pub fn create(context: &Rc<Class>, current_method: &Rc<MethodInfo>) -> Self {
        let max_locals = current_method.get_code_attribute().max_locals;
        Frame {
            pc: 0,
            local_variable: vec![0; max_locals as usize],
            operand_stack: vec![],
            context: Rc::clone(context),
            current_method: Rc::clone(current_method),
        }
    }
}
