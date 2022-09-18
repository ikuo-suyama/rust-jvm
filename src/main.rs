extern crate core;

mod binary;
mod class;
mod class_attributes;
mod class_file;
mod class_loader;
mod cp_info;
mod instruction_set;
mod interpreter;
mod jvm;
mod thread;

use crate::jvm::JVM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let vm = JVM::create();
    vm.launch(&args[1..]);
}
