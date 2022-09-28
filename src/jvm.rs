use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::class::Class;
use crate::class_attributes::MethodInfo;
use crate::class_loader::ClassLoader;
use crate::interpreter::interpret;
use crate::main;
use crate::thread::{Frame, Thread};

pub struct MethodArea {
    class_area: HashMap<String, Rc<Class>>,
    thread_area: HashMap<String, Rc<Thread>>,
}

impl MethodArea {
    pub fn create() -> MethodArea {
        MethodArea {
            class_area: HashMap::new(),
            thread_area: HashMap::new(),
        }
    }
    pub fn register_class(&mut self, class: Class) -> Rc<Class> {
        let class_ref = Rc::new(class);
        let result = Rc::clone(&class_ref);
        self.class_area
            .insert(class_ref.descriptor.clone(), class_ref);
        result
    }
    pub fn lookup_class(&mut self, name: String) -> Option<Rc<Class>> {
        self.class_area
            .get(name.as_str())
            .map(|class_ref| Rc::clone(class_ref))
    }
}

pub struct JVM {
    method_area: RefCell<MethodArea>,
    boot_loader: ClassLoader,
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
            method_area: RefCell::new(MethodArea::create()),
            boot_loader: ClassLoader {},
        }
    }

    pub fn launch(&mut self, args: &[String]) {
        println!("[DEBUG] -- {:?}", args);

        let class = self.boot_loader.load_class(&args[0]);

        self.invoke_main(class);
    }

    fn invoke_main(&mut self, class: Class) {
        let class_ref = self.method_area.get_mut().register_class(class);
        let main_method = find_main(&class_ref);

        let mut thread = Thread::create();

        let mut frame: Frame = Frame::create(&class_ref, &main_method);
        thread.java_virtual_machine_stack.push(frame);

        interpret(&mut thread);
    }
}

static MAIN_METHOD_NAME_DESCRIPTOR: &str = "main:([Ljava/lang/String;)V";
/// fetch target method as main for now
static STUB_MAIN_METHOD_NAME_DESCRIPTOR: &str = "main:()I";

fn find_main(class: &Class) -> Rc<MethodInfo> {
    // TODO: keep backward compatibility by STUB_MAIN_METHOD_NAME_DESCRIPTOR. remove this later
    let stub_method_ref = class.methods.get(STUB_MAIN_METHOD_NAME_DESCRIPTOR);
    let main_method_ref = class.methods.get(MAIN_METHOD_NAME_DESCRIPTOR);

    let method_ref = stub_method_ref
        .or(main_method_ref)
        .expect("Error: Can't Find Main Method. Please define Main Method as:\npublic static void main(String[] args)");

    Rc::clone(method_ref)
}
