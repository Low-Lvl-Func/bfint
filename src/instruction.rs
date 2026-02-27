#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    IncrementPtr,      // >
    DecrementPtr,      // <
    IncrementByte,     // +
    DecrementByte,     // -
    OutputByte,        // .
    InputByte,         // ,
    JumpForward(usize),  // [
    JumpBackward(usize), // ]
}