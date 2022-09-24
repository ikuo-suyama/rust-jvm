use crate::class_loader::ClassLoader;
use crate::interpreter::frame_test::dummy_class;
use crate::thread::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invoke_static(
    class_loader: &ClassLoader,
    frame_stack: &mut Vec<RefCell<Frame>>,
    current_frame: &mut Frame,
    methodref_cp_index: u16,
) {
    // 0. constantpool lookup
    // 1. class lookup
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

// #[test]
// pub fn test_invoke_static() {
//     let class_loader = ClassLoader {};
//     let frame_stack: Vec<RefCell<Frame>> = vec![];
//
//     let class = dummy_class();
//     current_frame = Frame::create()
//
// }
