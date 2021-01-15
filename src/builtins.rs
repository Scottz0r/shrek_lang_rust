use crate::shrek_vm::*;

use std::io::{self, Write};

// TODO: This woudl be used to reserve builtins when/if extensions are supported.
// pub const BUILTIN_MAX: i32 = 63;

pub mod ops {
    pub const INPUT: i32 = 0;
    pub const OUTPUT: i32 = 1;
    pub const ADD: i32 = 2;
    pub const SUBTRACT: i32 = 3;
    pub const MULTIPLY: i32 = 4;
    pub const DIVIDE: i32 = 5;
    pub const MOD_: i32 = 6;
    pub const DOUBLE_VAL: i32 = 7;
    pub const NEGATE: i32 = 8;
    pub const SQUARE: i32 = 9;
    pub const CLONE: i32 = 10;
}

pub fn execute_builtin(vm: &mut ShrekVM, func_num: i32) -> VmResult<()> {
    match func_num {
        ops::INPUT => input(vm, read_line_stdio, prompt_stdout),
        ops::OUTPUT => output(vm, write_stdout),
        ops::ADD => add(vm),
        ops::SUBTRACT => subtract(vm),
        ops::MULTIPLY => multiply(vm),
        ops::DIVIDE => divide(vm),
        ops::MOD_ => mod_(vm),
        ops::DOUBLE_VAL => double_val(vm),
        ops::NEGATE => negate(vm),
        ops::SQUARE => square(vm),
        ops::CLONE => clone(vm),
        _ => Err(ShrekRuntimeError::new("invalid builtin function number")),
    }
}

/// Simple function that will be used to dependency inject getting a string from stdin.
fn read_line_stdio() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Err(_) => None,
        Ok(_) => Some(buffer),
    }
}

/// Promper for user input. Used for dependency injection.
fn prompt_stdout() -> VmResult<()> {
    // Must flush this immediately, otherwise output will be buffered and not show this prompt.
    print!("input: ");
    match io::stdout().flush() {
        Err(_) => return Err(ShrekRuntimeError::new("i/o error writing to stdout")),
        _ => Ok(()),
    }
}

// Write a value to stdout. Used for dependency injection.
fn write_stdout(val: i32) {
    println!("{}", val);
}

fn input<I, P>(vm: &mut ShrekVM, read_line: I, prompt: P) -> VmResult<()>
where
    I: FnOnce() -> Option<String>,
    P: FnOnce() -> VmResult<()>,
{
    // Prompt for input (injected function). This can fail.
    prompt()?;

    // Read line (injected function). This can fail.
    let mut buffer = match read_line() {
        Some(x) => x,
        None => return Err(ShrekRuntimeError::new("Error reading input")),
    };

    // Readline includes newline char, so trim it out.
    buffer = buffer.trim_end().to_string();

    // Add a null terminator.
    vm.push(0);

    // Add the string in reverse order, so popping off the stack will be in the forward
    // direction. These will be added as raw bytes.
    for c in buffer.bytes().rev() {
        vm.push(c as i32);
    }

    Ok(())
}

fn output<O>(vm: &mut ShrekVM, ouput_func: O) -> VmResult<()>
where
    O: FnOnce(i32) -> (),
{
    let v0 = vm.peek()?;
    ouput_func(v0);
    Ok(())
}

fn add(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 2 {
        Err(ShrekRuntimeError::new("add requires 2 items on the stack"))
    } else {
        let v0 = vm.pop()?;
        let v1 = vm.pop()?;

        let val = v1 + v0;
        vm.push(val);

        Ok(())
    }
}

fn subtract(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 2 {
        Err(ShrekRuntimeError::new(
            "subtract requires 2 items on the stack",
        ))
    } else {
        let v0 = vm.pop()?;
        let v1 = vm.pop()?;

        let val = v1 - v0;
        vm.push(val);

        Ok(())
    }
}

fn multiply(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 2 {
        Err(ShrekRuntimeError::new(
            "multiply requires 2 items on the stack",
        ))
    } else {
        let v0 = vm.pop()?;
        let v1 = vm.pop()?;

        let val = v1 * v0;
        vm.push(val);

        Ok(())
    }
}

fn divide(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 2 {
        Err(ShrekRuntimeError::new(
            "divide requires 2 items on the stack",
        ))
    } else {
        let v0 = vm.pop()?;
        let v1 = vm.pop()?;

        let val = v1 / v0;
        vm.push(val);

        Ok(())
    }
}

fn mod_(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 2 {
        Err(ShrekRuntimeError::new("mod requires 2 items on the stack"))
    } else {
        let v0 = vm.pop()?;
        let v1 = vm.pop()?;

        let val = v1 % v0;
        vm.push(val);

        Ok(())
    }
}

fn double_val(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 1 {
        Err(ShrekRuntimeError::new(
            "double_val requires 1 item on the stack",
        ))
    } else {
        let v0 = vm.pop()?;

        let val = v0 * 2;
        vm.push(val);

        Ok(())
    }
}

fn negate(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 1 {
        Err(ShrekRuntimeError::new(
            "negate requires 1 item on the stack",
        ))
    } else {
        let v0 = vm.pop()?;

        let val = -v0;
        vm.push(val);

        Ok(())
    }
}

fn square(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 1 {
        Err(ShrekRuntimeError::new(
            "square requires 1 item on the stack",
        ))
    } else {
        let v0 = vm.pop()?;

        let val = v0 * v0;
        vm.push(val);

        Ok(())
    }
}

fn clone(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 1 {
        Err(ShrekRuntimeError::new("clone requires 1 item on the stack"))
    } else {
        let v0 = vm.peek()?;
        vm.push(v0);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_input() {
        let input_mock = || Some("asdf".to_string());
        let disp_mock = || -> VmResult<()> { Ok(()) };

        let mut vm = ShrekVM::new(Vec::new());

        input(&mut vm, input_mock, disp_mock).unwrap();

        assert_eq!('a' as i32, vm.pop().unwrap());
        assert_eq!('s' as i32, vm.pop().unwrap());
        assert_eq!('d' as i32, vm.pop().unwrap());
        assert_eq!('f' as i32, vm.pop().unwrap());
        assert_eq!(0, vm.pop().unwrap());

        assert_eq!(0, vm.count());
    }

    #[test]
    fn test_output() {
        let disp_mock = |_| ();

        let mut vm = ShrekVM::new(Vec::new());
        vm.push(123);

        output(&mut vm, disp_mock).unwrap();

        assert_eq!(123, vm.peek().unwrap());
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_add() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(1);
        vm.push(3);

        add(&mut vm).unwrap();

        assert_eq!(4, vm.peek().unwrap());
        // Two values added above should be popped.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_add_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        vm.push(3);
        assert!(add(&mut vm).is_err());
    }

    #[test]
    fn test_subtract() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(1);
        vm.push(3);

        subtract(&mut vm).unwrap();

        assert_eq!(-2, vm.peek().unwrap());
        // Two values added above should be popped.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_subtract_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        vm.push(3);
        assert!(subtract(&mut vm).is_err());
    }

    #[test]
    fn test_multiply() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(8);
        vm.push(3);

        multiply(&mut vm).unwrap();

        assert_eq!(24, vm.peek().unwrap());
        // Two values added above should be popped.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_multiply_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        vm.push(3);
        assert!(multiply(&mut vm).is_err());
    }

    #[test]
    fn test_divide() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(13);
        vm.push(3);

        divide(&mut vm).unwrap();

        assert_eq!(4, vm.peek().unwrap());
        // Two values added above should be popped.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_divide_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        vm.push(3);
        assert!(divide(&mut vm).is_err());
    }

    #[test]
    fn test_mod_() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(13);
        vm.push(3);

        mod_(&mut vm).unwrap();

        assert_eq!(1, vm.peek().unwrap());
        // Two values added above should be popped.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_mod_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        vm.push(3);
        assert!(mod_(&mut vm).is_err());
    }

    #[test]
    fn test_double_val() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(3);

        double_val(&mut vm).unwrap();

        assert_eq!(6, vm.peek().unwrap());
        // Value on stack should be replaced with result.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_double_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        assert!(double_val(&mut vm).is_err());
    }

    #[test]
    fn test_negate() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(3);

        negate(&mut vm).unwrap();

        assert_eq!(-3, vm.peek().unwrap());
        // Value on stack should be replaced with result.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_negate_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        assert!(negate(&mut vm).is_err());
    }

    #[test]
    fn test_square() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(4);

        square(&mut vm).unwrap();

        assert_eq!(16, vm.peek().unwrap());
        // Value on stack should be replaced with result.
        assert_eq!(1, vm.count());
    }

    #[test]
    fn test_square_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        assert!(square(&mut vm).is_err());
    }

    #[test]
    fn test_clone() {
        let mut vm = ShrekVM::new(Vec::new());

        vm.push(3);

        clone(&mut vm).unwrap();

        assert_eq!(3, vm.peek().unwrap());
        // Initial pushed value should be kept.
        assert_eq!(2, vm.count());
    }

    #[test]
    fn test_clone_bad_stack() {
        let mut vm = ShrekVM::new(Vec::new());
        assert!(clone(&mut vm).is_err());
    }
}
