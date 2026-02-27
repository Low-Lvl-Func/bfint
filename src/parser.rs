use crate::instruction::Instruction;

pub fn parse(code: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut stack = Vec::new();

    for c in code.chars().filter(|c| "><+-.,[]".contains(*c)) {
        match c {
            '>' => instructions.push(Instruction::IncrementPtr),
            '<' => instructions.push(Instruction::DecrementPtr),
            '+' => instructions.push(Instruction::IncrementByte),
            '-' => instructions.push(Instruction::DecrementByte),
            '.' => instructions.push(Instruction::OutputByte),
            ',' => instructions.push(Instruction::InputByte),
            '[' => {
                stack.push(instructions.len());
                instructions.push(Instruction::JumpForward(0));
            }
            ']' => {
                let start = stack.pop().expect("Mismatched brackets: too many ]");
                let end = instructions.len();
                instructions[start] = Instruction::JumpForward(end);
                instructions.push(Instruction::JumpBackward(start));
            }
            _ => unreachable!(),
        }
    }
    instructions
}