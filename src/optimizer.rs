use std::vec::Vec;
use crate::byte_code::*;
use crate::builtins;

const MAX_OPTIMIZE_LOOPS: i32 = 32;

pub fn optimize(code: &Vec<ByteCode>) -> Vec<ByteCode> {
    // Must optimize easy constants before attempting to compress arithmetic.
    let mut result = optimize_easy_constants(code);

    let mut counter = 0;
    loop {
        let mut is_optimizing = false;

        match optimize_1_arg_arithmetic(&result) {
            Some(optimized) => { is_optimizing = true; result = optimized; },
            None => ()
        }
        
        match optimize_2_arg_arithmetic(&result) {
            Some(optimized) => { is_optimizing = true; result = optimized },
            None => ()
        };

        counter += 1;

        if !is_optimizing || counter >= MAX_OPTIMIZE_LOOPS {
            break;
        }
    }

    result
}

/// Optimize code that is a Push0 then a chain of bumps. This will compress the operation into a 
/// single push constant with the bumps combined into a single arg.
fn optimize_easy_constants(code: &Vec<ByteCode>) -> Vec<ByteCode> {
    let mut result = Vec::new();
    
    let mut push_index: Option<usize> = None;
    let mut bump_value = 0;

    for i in 0..code.len() {
        // Check if the bump chain of bumping has ended before handling current operation.
        if push_index.is_some() && code[i].op_code != OpCode::Bump {
            // If there was a push and it was bumped, then it can be simplified into a single operation of
            // pushing a constant to the stack. Replace the push0 and bumps with a single operation.
            if bump_value > 0 {
                result.push(ByteCode{ op_code: OpCode::PushConst, arg: bump_value });
            } else {
                // There were no bumps, so the push0 needs to be copied to the output code.
                result.push(code[push_index.unwrap()].clone());
            }

            push_index = None;
            bump_value = 0;
        }

        if code[i].op_code == OpCode::Push0 {
            bump_value = 0;
            push_index = Some(i);
        } else if push_index.is_some() && code[i].op_code == OpCode::Bump {
            bump_value += 1;
        } else {
            result.push(code[i].clone());
        }
    }

    // A non None value in the push index indicates the last operation was a push, which
    // needs to be added.
    if push_index.is_some() {
        result.push(code.last().unwrap().clone());
    }

    result
}

/// Optimize code like the following to a single constant. This assumes that "easy constant" optimization has been
/// executed.
///
/// ```
/// Push Constant <= v1 (stack top - 1)
/// Push Constant <= v0 (stack top when func executing)
/// Push Constant <= If this constant is an arithmetic function.
/// Function Call
/// ```
/// 
// This series of commands can be turned into a single constant because arithmetic on constants will always be
/// a constant value. This will cover cases where two constants are "mathed" into a single constant.
fn optimize_2_arg_arithmetic(code: &Vec<ByteCode>) -> Option<Vec<ByteCode>> {
    // If there are not enough operations in the code, do not attempt to optimize.
    if code.len() < 4 {
        return None;
    }

    let mut result = Vec::new();
    let mut i: usize = 0;

    while i < code.len() {
        // If there are not enough operations to loop forward, stop trying to optimize. Below logic assumes there
        // will always be at least 4 codes to inspect.
        if i > code.len() - 4 {
            result.push(code[i].clone());
            i += 1;
            continue;
        }

        // Need 3 push constants - two for the value and one for the function number.
        let mut has_push_const = code[i].op_code == OpCode::PushConst;
        has_push_const &= code[i + 1].op_code == OpCode::PushConst;
        has_push_const &= code[i + 2].op_code == OpCode::PushConst;

        let func_num = code[i + 2].arg;
        let has_arithmetic = is_two_arg_arithmetic(&code[i + 3], func_num);
        let mut was_replaced = false;

        if has_push_const && has_arithmetic {
            // v0 and v1 are the low indexes becuase this is looking foward from i. So, i will be the "bottom" of the stack
            // for this operation.
            let v0 = code[i + 1].arg;
            let v1 = code[i].arg;

            let mut r = 0;
            let mut do_replace = true;

            match code[i].arg {
                builtins::ops::ADD => r = v1 + v0,
                builtins::ops::SUBTRACT => r = v1 - v0,
                builtins::ops::DIVIDE => r = v1 / v0,
                builtins::ops::MOD_ => r = v1 % v0,
                _ => do_replace = false
            }

            if do_replace {
                result.push(ByteCode{ op_code: OpCode::PushConst, arg: r });
                was_replaced = true;

                // Jump i by four operations to the next unoptimized code.
                i += 4;
            }
        }

        // No optimization was found looking forward, so add the current op to the result.
        if !was_replaced {
            result.push(code[i].clone());
            i += 1;
        }
    }

    if result.len() < code.len() {
        Some(result)
    } else {
        None
    }
}

fn is_two_arg_arithmetic(byte_code: &ByteCode, func_num: i32) -> bool {
    if byte_code.op_code == OpCode::Func {
        match func_num {
            builtins::ops::ADD | builtins::ops::SUBTRACT | builtins::ops::DIVIDE | builtins::ops::MOD_ => true,
            _ => false
        }
    } else {
        false
    }
}

/// Optimize code like the following to a single constant. This assumes that "easy constant" optimization has been
/// executed.
///
/// ```
/// Push Constant <= v0 (stack top when func executing)
/// Push Constant <= If this constant is an arithmetic function.
/// Function Call
/// ```
///
/// This series of commands can be turned into a single constant because arithmetic on constants will always be
/// a constant value. This will cover cases where two constants are "mathed" into a single constant.
fn optimize_1_arg_arithmetic(code: &Vec<ByteCode>) -> Option<Vec<ByteCode>> {
    if code.len() < 3 {
        return None;
    }

    let mut result = Vec::new();
    let mut i: usize = 0;

    while i < code.len() {
        // If there are not enough operations to loop forward, stop trying to optimize. Below logic assumes there
        // will always be at least 3 codes to inspect.
        if i > code.len() - 3 {
            result.push(code[i].clone());
            i += 1;
            continue;
        }

        let mut has_push_const = code[i].op_code == OpCode::PushConst;
        has_push_const &= code[i + 1].op_code == OpCode::PushConst;

        let func_num = code[i + 1].arg;
        let has_arithmetic = is_one_arg_arithmetic(&code[i + 2], func_num);

        let mut was_replaced = false;
        if has_push_const && has_arithmetic {
            let v0 = code[i].arg;
            let mut r = 0;
            let mut do_replace = true;

            match func_num {
                builtins::ops::DOUBLE_VAL => r = v0 * 2,
                builtins::ops::NEGATE => r = v0 * -1,
                builtins::ops::SQUARE => r = v0 * v0,
                _ => do_replace = false
            }

            if do_replace {
                result.push(ByteCode{op_code: OpCode::PushConst, arg: r});
                was_replaced = true;

                // Jump i by three operations to the next unoptimized code.
                i += 3;
            }
        }

        if !was_replaced {
            result.push(code[i].clone());
            i += 1;
        }
    }

    if result.len() < code.len() {
        Some(result)
    } else {
        None
    }
}

fn is_one_arg_arithmetic(byte_code: &ByteCode, func_num: i32) -> bool {
    if byte_code.op_code == OpCode::Func {
        match func_num {
            builtins::ops::DOUBLE_VAL | builtins::ops::NEGATE | builtins::ops::SQUARE => true,
            _ => false
        }
    } else {
        false
    }
}
