use crate::class_attributes::{FieldInfo, MethodInfo};
use crate::class_loader::ClassLoader;
use crate::java_class::ClassFile;
use crate::thread::Frame;
use std::collections::HashMap;

pub struct JVM {
    method_area: HashMap<String, Class>,
    boot_loader: ClassLoader,
}

pub struct Class {
    methods: HashMap<String, MethodInfo>,
    fields: HashMap<String, FieldInfo>,
}

impl Class {
    fn createFrom(class_file: ClassFile) -> Class {
        Class {
            methods: HashMap::new(),
            fields: HashMap::new(),
        }
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
        println!("DEBUG -- {:?}", args);

        let class_file = self.boot_loader.load_class(&args[0]);
        let class = Class::createFrom(class_file);

        let code = vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];

        let frame = Frame {};

        frame.invoke(code);
    }
}
