use std::env;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    IncrementPtr,      // >
    DecrementPtr,      // <
    IncrementByte,     // +
    DecrementByte,     // -
    OutputByte,        // .
    InputByte,         // ,
    JumpForward(usize),  // [ -> Jump to matching ]
    JumpBackward(usize), // ] -> Jump back to matching [
}

struct BrainfuckVM {
    memory: Vec<u8>,
    pointer: usize,
    program: Vec<Instruction>,
    pc: usize,
}

impl BrainfuckVM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            memory: vec![0; 30000],
            pointer: 0,
            program,
            pc: 0,
        }
    }

    fn run(&mut self) {
        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Instruction::IncrementPtr => {
                    self.pointer = (self.pointer + 1) % self.memory.len();
                }
                Instruction::DecrementPtr => {
                    if self.pointer == 0 {
                        self.pointer = self.memory.len() - 1;
                    } else {
                        self.pointer -= 1;
                    }
                }
                Instruction::IncrementByte => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                }
                Instruction::DecrementByte => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                }
                Instruction::OutputByte => {
                    let _ = stdout.write_all(&[self.memory[self.pointer]]);
                    let _ = stdout.flush();
                }
                Instruction::InputByte => {
                    let mut buf = [0u8; 1];
                    if stdin.read_exact(&mut buf).is_ok() {
                        self.memory[self.pointer] = buf[0];
                    }
                }
                Instruction::JumpForward(target) => {
                    if self.memory[self.pointer] == 0 {
                        // If cell is 0, jump to the instruction AFTER the ]
                        self.pc = target;
                    }
                }
                Instruction::JumpBackward(target) => {
                    if self.memory[self.pointer] != 0 {
                        // If cell is NOT 0, jump back to the [ instruction
                        self.pc = target;
                    }
                }
            }
            self.pc += 1;
        }
    }
}

fn parse(code: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut stack = Vec::new();

    // Standard Brainfuck only cares about these 8 characters
    let filtered_chars: Vec<char> = code.chars()
        .filter(|c| "><+-.,[]".contains(*c))
        .collect();

    for (_i, c) in filtered_chars.iter().enumerate() {
        match c {
            '>' => instructions.push(Instruction::IncrementPtr),
            '<' => instructions.push(Instruction::DecrementPtr),
            '+' => instructions.push(Instruction::IncrementByte),
            '-' => instructions.push(Instruction::DecrementByte),
            '.' => instructions.push(Instruction::OutputByte),
            ',' => instructions.push(Instruction::InputByte),
            '[' => {
                stack.push(instructions.len());
                instructions.push(Instruction::JumpForward(0)); // Placeholder
            }
            ']' => {
                let start = stack.pop().expect("Mismatched brackets: too many ]");
                let end = instructions.len();
                // [ jumps to ]
                instructions[start] = Instruction::JumpForward(end);
                // ] jumps back to [
                instructions.push(Instruction::JumpBackward(start));
            }
            _ => unreachable!(),
        }
    }

    if !stack.is_empty() {
        panic!("Mismatched brackets: too many [");
    }

    instructions
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }

    let input = &args[1];
    let code = if input.ends_with(".bf") {
        fs::read_to_string(input).unwrap_or_else(|_| input.to_string())
    } else {
        input.to_string()
    };

    let program = parse(&code);
    let mut vm = BrainfuckVM::new(program);
    vm.run();
    println!();
}