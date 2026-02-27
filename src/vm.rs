use std::io::{self, Read, Write};
use crate::instruction::Instruction;

const DEFAULT_MEMORY_SIZE: usize = 30_000;

pub struct BrainfuckVM {
    memory: Vec<u8>,
    pointer: usize,
    program: Vec<Instruction>,
    pc: usize,
}

impl BrainfuckVM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            memory: vec![0; DEFAULT_MEMORY_SIZE],
            pointer: 0,
            program,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Instruction::IncrementPtr => {
                    self.pointer = (self.pointer + 1) % self.memory.len();
                }
                Instruction::DecrementPtr => {
                    self.pointer = if self.pointer == 0 { self.memory.len() - 1 } else { self.pointer - 1 };
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
                    if self.memory[self.pointer] == 0 { self.pc = target; }
                }
                Instruction::JumpBackward(target) => {
                    if self.memory[self.pointer] != 0 { self.pc = target; }
                }
            }
            self.pc += 1;
        }
    }
}