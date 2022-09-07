mod jvm;
mod thread;

use crate::jvm::JVM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let vm = JVM::create();
    vm.launch();
}
