use crate::class::ClassMeta;
use crate::class_attributes::MethodInfo;
use crate::types::JVMTypes;
use std::rc::Rc;

pub struct Thread {
    pub java_virtual_machine_stack: Vec<Frame>,
}

impl Thread {
    pub fn create() -> Self {
        Thread {
            java_virtual_machine_stack: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub pc: u64,
    pub local_variable: Vec<JVMTypes>,
    pub operand_stack: Vec<JVMTypes>,
    pub context: Rc<ClassMeta>,
    pub current_method: Rc<MethodInfo>,
}

impl Frame {
    pub fn create(context: &Rc<ClassMeta>, current_method: &Rc<MethodInfo>) -> Self {
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
