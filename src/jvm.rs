use crate::classloader::ClassLoader;
use crate::thread::Frame;

pub struct JVM {}

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
        JVM {}
    }

    pub fn launch(self, args: &[String]) {
        println!("DEBUG -- {:?}", args);
        let class_loader = ClassLoader {};
        let code = class_loader.load_class(&args[0]);

        let frame = Frame {};

        frame.invoke(code);
    }
}
