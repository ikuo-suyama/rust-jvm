use std::collections::HashMap;

use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::class_loader::ClassLoader;
use crate::interpreter::interpret;
use crate::main;
use crate::thread::{Frame, Thread};

pub struct JVM {
    method_area: HashMap<String, Class>,
    boot_loader: ClassLoader,
}

trait Interpreter {
    fn interpret<'a>(
        &'a self,
        thread: &'a mut Thread<'a>,
        context: &'a Class,
        method: &'a MethodInfo,
    ) {
    }
}

impl Interpreter for JVM {
    fn interpret<'a>(
        &'a self,
        thread: &'a mut Thread<'a>,
        context: &'a Class,
        method: &'a MethodInfo,
    ) {
        let frame: Frame<'a> = Frame::create(context, method);

        thread.java_virtual_machine_stack.push(frame);

        let top = thread.java_virtual_machine_stack.len() - 1;
        let frame = thread.java_virtual_machine_stack.get_mut(top).unwrap();

        interpret(frame);
    }
}

/// The first primitive JVM. A simple instruction interpreter.
/// The code array is obtained by the following procedure.
///
/// Compile SimpleSum.java, which performs a simple addition calculation.
/// ```sh
/// $ cd java/
/// $ javac src/SimpleSum.java
/// ````
///
/// 2. extract the Code part of MethodAttribute from SimpleSum.class.
///
/// 3. implement it by comparing it with the one decompiled into machine language.
/// ```sh
/// $ cd java/
/// $ javap -v -p -s -constants SimpleSum.class > SimpleSum.jvm
/// ```
/// Instruction Spec
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html
impl JVM {
    pub fn create() -> Self {
        JVM {
            method_area: HashMap::new(),
            boot_loader: ClassLoader {},
        }
    }

    pub fn launch(mut self, args: &[String]) {
        println!("[DEBUG] -- {:?}", args);

        let class = self.boot_loader.load_class(&mut self.method_area, &args[0]);
        // println!("[DEBUG] -- {:#?}", class);

        let main_method = find_main(class);

        let mut thread = Thread::create();
        // thread.run(class, main_method);

        // self.interpret(&mut thread, &class, &main_method);
        let frame: Frame = Frame::create(class, main_method);

        thread.java_virtual_machine_stack.push(frame);

        let top = thread.java_virtual_machine_stack.len() - 1;
        let frame = thread.java_virtual_machine_stack.get_mut(top).unwrap();

        interpret(frame);
    }
}

// static MAIN_METHOD_NAME_DESCRIPTOR: &str = "main:([Ljava/lang/String;)V";
/// fetch target method as main for now
static MAIN_METHOD_NAME_DESCRIPTOR: &str = "main:()I";

fn find_main(class: &Class) -> &MethodInfo {
    &class.methods
        .get(MAIN_METHOD_NAME_DESCRIPTOR)
        .expect("Error: Can't Find Main Method. Please define Main Method as:\npublic static void main(String[] args)")
}
