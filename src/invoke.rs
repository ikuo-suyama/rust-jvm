use crate::class::MethodRef;
use crate::class_loader::ClassLoader;
use crate::cp_info::constant_pool_value_at;
use crate::interpreter::interpret;
use crate::thread::{Frame, Thread};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::Chars;

fn parse_descriptor(descriptor: &String) -> String {
    let arguments = String::from(descriptor.trim_start_matches("("));
    let arguments: Vec<&str> = arguments.split(")").collect();
    String::from(arguments[0])
}

pub fn invoke_static(
    // class_loader: &ClassLoader,
    thread: &mut Thread,
    methodref_cp_index: u16,
) {
    // 0. class lookup
    // TODO: from ClassLoader
    let current_frame = thread.java_virtual_machine_stack.last_mut().unwrap();
    let class = &current_frame.context;

    // 1. constantpool lookup
    let method_descriptor = class.constant_pool_value_at(methodref_cp_index);
    let method_ref = MethodRef::parse_from(method_descriptor);

    // 2. method lookup
    let method_info = class
        .methods
        .get(method_ref.name_and_descriptor.as_str())
        .expect(format!("Method Not Found: {}", method_ref.name_and_descriptor).as_str());

    // 3. create new frame, push arguments as local_val
    let mut invoked_frame = Frame::create(class, method_info);

    // 4. pop arguments val from current frame operand_stack, set it to new frame's local val
    let descriptor = method_ref.descriptor;
    // TODO: parser, multiple args
    let args = parse_descriptor(&descriptor);
    if args.len() > 0 {
        let arg = current_frame.operand_stack.pop().unwrap();
        invoked_frame.local_variable[0] = arg;
    }

    // 5. push to java_stack
    thread.java_virtual_machine_stack.push(invoked_frame);

    println!(
        "\n[DEBUG] -- >>>> invoke_static: {}.{}",
        method_ref.class, method_ref.name_and_descriptor
    )
}

pub fn i_return(thread: &mut Thread, returned_value: u64) {
    println!("[DEBUG] -- <<<< i_return: value: {}", returned_value);

    // 1. pop current frame from stack
    let _ = thread.java_virtual_machine_stack.pop().unwrap();

    // 2. get previous frame as invoker
    if let Some(invoker_stack) = thread.java_virtual_machine_stack.last_mut() {
        // 3. push returned value to invoker frame operand_stack
        invoker_stack.operand_stack.push(returned_value);
    }
}

pub fn java_return(thread: &mut Thread) {
    println!("[DEBUG] -- <<<< return");

    // 1. pop current frame from stack
    let _ = thread.java_virtual_machine_stack.pop().unwrap();
}

#[test]
pub fn test_invoke_static() {
    use crate::instruction::frame_test::{dummy_class, dummy_method};

    // let class_loader = ClassLoader {};
    let mut thread = Thread::create();

    let mut class = dummy_class();
    let full_method_name = String::from("Dummy.main:()I");
    let cp = vec![String::from(""), full_method_name.clone()];
    class.runtime_constant_pool = cp;

    // icnost_2, ireturn
    let method_name = String::from("main:()I");
    let code: Vec<u8> = vec![0x5, 0xac];
    let method_info = Rc::new(dummy_method(code));
    class.methods.insert(method_name, Rc::clone(&method_info));

    let mut current_frame = Frame::create(&Rc::new(class), &Rc::clone(&method_info));
    thread.java_virtual_machine_stack.push(current_frame);
    let mr_index: u16 = 1;

    invoke_static(
        // &class_loader,
        &mut thread,
        mr_index,
    );

    assert_eq!(thread.java_virtual_machine_stack.len(), 2);
}
