mod jvm;
mod thread;

use crate::jvm::JVM;

fn main() {
    let vm = JVM::create();
    vm.launch();
}
