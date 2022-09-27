use crate::binary::debug_bytes;
use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::interpreter::interpret;
use crate::JVM;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct Thread {
    pub java_virtual_machine_stack: RefCell<Vec<RefCell<Frame>>>,
}

impl Thread {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: RefCell::new(vec![]),
        }
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
