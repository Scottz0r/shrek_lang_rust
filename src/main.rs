mod builtins;
mod byte_code;
mod optimizer;
mod shrek_parser;
mod shrek_vm;

use std::env;
use std::fs;

use byte_code::ByteCode;
use shrek_parser::*;
use shrek_vm::ShrekVM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Invalid arguments. Expected source file.");
        std::process::exit(1);
    }

    let input_code = match fs::read_to_string(&args[1]) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Error reading source file: {:?}", err);
            std::process::exit(1);
        }
    };

    let byte_code = match parse_code(&input_code) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Parse error: {:?}", err);
            std::process::exit(1);
        }
    };

    let mut vm = ShrekVM::new(byte_code);
    let exit_code = match vm.run() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Shrek RuntimeError: {:?}", err);
            3
        }
    };

    std::process::exit(exit_code);
}

fn parse_code(input_code: &str) -> ParseResult<Vec<ByteCode>> {
    let tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(&input_code)?;
    let syntax_tree = SyntaxTree::generate(&tokens)?;
    let byte_code = generate_byte_code(&syntax_tree)?;

    Ok(optimizer::optimize(&byte_code))
}
