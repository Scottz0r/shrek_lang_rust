use crate::byte_code::{ByteCode, OpCode};
use crate::builtins;
use std::vec::Vec;
use std::collections::HashMap;
use std::fmt;

pub struct ShrekVM {
    byte_code: Vec<ByteCode>,

    program_counter: usize,
    stack: Vec<i32>,

    jump_table: HashMap<i32, usize>
}

#[derive(Debug, Clone)]
pub struct ShrekRuntimeError {
    pub message: String
}

pub type VmResult<T> = Result<T, ShrekRuntimeError>;

impl ShrekVM {
    pub fn new(byte_code: Vec<ByteCode>) -> ShrekVM {
        ShrekVM {
            byte_code,
            program_counter: 0,
            stack: Vec::new(),
            jump_table: HashMap::new()
        }
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }
    
    pub fn pop(&mut self) -> VmResult<i32> {
        match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(ShrekRuntimeError::new("cannot pop: stack is empty"))
        }
    }

    pub fn peek(&self) -> VmResult<i32> {
        match self.stack.last() {
            Some(x) => Ok(*x),
            None => Err(ShrekRuntimeError::new("cannot peek: stack is empty"))
        }
    }

    pub fn count(&self) -> usize {
        self.stack.len()
    }

    pub fn run(&mut self) -> VmResult<i32> {
        self.build_jump_table();

        while self.program_counter < self.byte_code.len() {
            self.step()?;
        }

        let exit_code = match self.stack.pop() {
            Some(x) => x,
            None => 0,
        };

        Ok(exit_code)
    }

    fn build_jump_table(&mut self) {
        for i in 0..self.byte_code.len() {
            let op = &self.byte_code[i];
            if op.op_code == OpCode::Label {
                self.jump_table.insert(op.arg, i);
            }
        }
    }

    fn step(&mut self) -> VmResult<()> {
        // TODO: This should be an assert
        if self.program_counter >= self.byte_code.len() {
            return Ok(());
        }

        match self.byte_code[self.program_counter].op_code {
            OpCode::Label => { self.step_code(); },
            OpCode::Push0 => self.op_push0()?,
            OpCode::Pop => self.op_pop()?,
            OpCode::Bump => self.op_bump()?,
            OpCode::Func => self.op_func()?,
            OpCode::Jump => self.op_jump()?,
            OpCode::PushConst => self.op_push_const()?,
            _ => ()
        }

        Ok(())
    }

    fn step_code(&mut self) {
        self.program_counter += 1;
    }

    fn op_push0(&mut self) -> VmResult<()> {
        self.push(0);
        self.step_code();
        Ok(())
    }

    fn op_pop(&mut self) -> VmResult<()> {
        let _ = self.pop()?;
        self.step_code();
        Ok(())
    }

    fn op_bump(&mut self) -> VmResult<()> {
        let mut v = self.pop()?;
        v += 1;
        self.push(v);
        self.step_code();
        Ok(())
    }

    fn op_func(&mut self) -> VmResult<()> {
        let func_num = self.pop()?;
        builtins::execute_builtin(self, func_num)?;
        self.step_code();
        Ok(())
    }

    fn op_jump(&mut self) -> VmResult<()> {
        // Assumes this function will not be called when program counter beyond code.
        debug_assert!(self.program_counter < self.byte_code.len());

        // Pop the top of the stack to get the jump type to do.
        let jump_type = self.pop()?;

        // Based on the jump type, determine if the jump should happen.
        let should_jump = match jump_type {
            0 => {
                // Normal Jump
                true
            },
            1 => {
                // Jump if 0
                let s1 = self.peek()?;
                s1 == 0
            },
            2 => {
                // Jump if negative
                let s1 = self.peek()?;
                s1 < 0
            },
            _ => { 
                return Err(ShrekRuntimeError::new("invalid jump type"));
            }
        };

        // No jump should happen.
        if !should_jump {
            self.step_code();
            return Ok(());
        }

        // Get the label to jump to from the label map. The code's argument will hold the label number.
        let label_num = self.byte_code[self.program_counter].arg;
        match self.jump_table.get(&label_num) {
            Some(x) => {
                // Move program counter 1 past the label to save an opeartion.
                self.program_counter = (*x as usize) + 1;
                Ok(())
            },
            None => Err(ShrekRuntimeError::new("jump label not found in jump map"))
        }
    }

    fn op_push_const(&mut self) -> VmResult<()> {
        // Assumes this function will not be called when program counter beyond code.
        debug_assert!(self.program_counter < self.byte_code.len());

        self.push(self.byte_code[self.program_counter].arg);
        self.step_code();
        Ok(())
    }
}

impl ShrekRuntimeError {
    pub fn new(message: &str) -> ShrekRuntimeError {
        ShrekRuntimeError{ message: message.to_string() }
    }
}

impl fmt::Display for ShrekRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.message)
    }
}
