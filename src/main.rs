extern crate core;

mod binary;
mod class_loader;
mod cp_info;
mod java_class;
mod jvm;
mod thread;

use crate::jvm::JVM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let vm = JVM::create();
    vm.launch(&args[1..]);
}
