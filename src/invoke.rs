use crate::class_loader::ClassLoader;
use crate::cp_info::constant_pool_value_at;
use crate::interpreter::frame_test::{dummy_class, dummy_method};
use crate::thread::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invoke_static(
    // class_loader: &ClassLoader,
    frame_stack: &mut Vec<RefCell<Frame>>,
    current_frame: &mut Frame,
    methodref_cp_index: u16,
) {
    // 0. class lookup
    // let class = current_frame.context;
    // 1. constantpool lookup
    // let methodref_cp = constant_pool_value_at(class.methods)
    // 2. method lookup
    // 3. pop arguments val from current frame operand_stack
    // 4. create new frame, push arguments as local_val
    // 5. push to java_stack
    // 6. interprit
}

pub fn i_return(frame_stack: &mut Vec<RefCell<Frame>>, current_frame: &mut Frame) {
    // 1. pop from current frame operand stack
    // 2. pop frame from frame_stack
    // 3. get previous frame as invoker
    // 4. push returned value to invoker frame
    // 5. resume interprit
}

#[test]
pub fn test_invoke_static() {
    let class_loader = ClassLoader {};
    let mut frame_stack: Vec<RefCell<Frame>> = vec![];

    let class = dummy_class();
    let code: Vec<u8> = vec![0x12];
    let method_info = dummy_method(code);
    let mut current_frame = Frame::create(&Rc::new(class), &Rc::new(method_info));
    let mr_index: u16 = 1;

    // invoke_static(&class_loader,
    //               &mut frame_stack,
    //               &mut current_frame,
    //               mr_index)
}
