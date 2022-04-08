use crate::jvm::JVM;

mod jvm;

fn main() {
    let vm = JVM::create();
    vm.launch();
}
