mod binary;
mod class;
mod class_attributes;
mod class_file;
mod class_loader;
mod cp_info;
mod instruction;
mod instruction_set;
mod interpreter;
mod invoke;
mod jvm;
mod thread;
mod types;

use crate::jvm::JVM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = JVM::create();
    vm.launch(&args[1..]);
}
