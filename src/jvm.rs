use crate::class::Class;
use crate::class_attributes::AttributeInfo::CodeAttributeInfo;
use crate::class_attributes::{AttributeInfo, FieldInfo, MethodInfo};
use crate::class_file::ClassFile;
use crate::class_loader::ClassLoader;
use crate::main;
use crate::thread::Frame;
use std::collections::HashMap;

pub struct JVM {
    method_area: HashMap<String, Class>,
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
            method_area: HashMap::new(),
            boot_loader: ClassLoader {},
        }
    }

    pub fn launch(mut self, args: &[String]) {
        println!("[DEBUG] -- {:?}", args);

        let class = self.boot_loader.load_class(&mut self.method_area, &args[0]);
        // println!("[DEBUG] -- {:#?}", class);

        let main_method = find_main(class);

        let frame = Frame::create(class, main_method);
        frame.invoke();
    }
}

// static MAIN_METHOD_NAME_DESCRIPTOR: &str = "main:([Ljava/lang/String;)V";
/// fetch target method as main for now
static MAIN_METHOD_NAME_DESCRIPTOR: &str = "sum:()I";

fn find_main(class: &Class) -> &MethodInfo {
    &class.methods
        .get(MAIN_METHOD_NAME_DESCRIPTOR)
        .expect("Error: Can't Find Main Method. Please define Main Method as:\npublic static void main(String[] args)")
}
