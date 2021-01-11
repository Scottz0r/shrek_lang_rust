use crate::shrek_vm::*;

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
        ops::INPUT => input(vm),
        ops::OUTPUT => output(vm),
        ops::ADD => add(vm),
        ops::SUBTRACT => subtract(vm),
        ops::MULTIPLY => multiply(vm),
        ops::DIVIDE => divide(vm),
        ops::MOD_ => mod_(vm),
        ops::DOUBLE_VAL => double_val(vm),
        ops::NEGATE => negate(vm),
        ops::SQUARE => square(vm),
        ops::CLONE => clone(vm),
        _ => Err(ShrekRuntimeError::new("invalid builtin function number"))
    }
}

fn input(vm: &mut ShrekVM) -> VmResult<()> {
    // TODO: Read from stdin
    Ok(())
}

fn output(vm: &mut ShrekVM) -> VmResult<()> {
    let v0 = vm.peek()?;
    println!("{}", v0);
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
        Err(ShrekRuntimeError::new("subtract requires 2 items on the stack"))
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
        Err(ShrekRuntimeError::new("multiply requires 2 items on the stack"))
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
        Err(ShrekRuntimeError::new("divide requires 2 items on the stack"))
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
        Err(ShrekRuntimeError::new("double_val requires 1 item on the stack"))
    } else {
        let v0 = vm.pop()?;

        let val = v0 * 2;
        vm.push(val);
        
        Ok(())
    }
}

fn negate(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 1 {
        Err(ShrekRuntimeError::new("negate requires 1 item on the stack"))
    } else {
        let v0 = vm.pop()?;

        let val = -v0;
        vm.push(val);
        
        Ok(())
    }
}

fn square(vm: &mut ShrekVM) -> VmResult<()> {
    if vm.count() < 1 {
        Err(ShrekRuntimeError::new("square requires 1 item on the stack"))
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
