mod instruction;
mod parser;
mod vm;

use std::{env, fs};
use vm::BrainfuckVM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: bf_interpreter <source_file.bf | code_string>");
        return;
    }

    let input = &args[1];
    let code = if input.ends_with(".bf") {
        fs::read_to_string(input).expect("Could not read file")
    } else {
        input.to_string()
    };

    let program = parser::parse(&code);
    let mut vm = BrainfuckVM::new(program);

    vm.run();
    println!(); // Ensure final newline
}