mod shrek_parser;
mod byte_code;
mod shrek_vm;
mod builtins;
mod optimizer;

use std::fs;
use std::path::Path;
use std::io;

use shrek_parser::*;
use shrek_vm::ShrekVM;

fn main() {
    // TODO: Read this from file.
    let test_data = read_file("demo.shrek").unwrap();

    let tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(&test_data).unwrap(); // TODO: Error checking.

    // for t in tokens.iter() {
    //     println!("Found {:?} token {:?} at {:?}", t.token_type, t.value, t.index);
    // }

    let syntax_tree = SyntaxTree::generate(&tokens).unwrap();

    // for stx in syntax_tree.tree.iter() {
    //     println!("Found token {:?}", stx.token.value);
    //     for cstx in stx.children.iter() {
    //         println!("  -> {:?}", cstx.token.value);
    //     }
    // }

    let mut byte_code = generate_byte_code(&syntax_tree).unwrap();

    println!("Unoptimized byte code ({} ops):", byte_code.len());
    for bc in byte_code.iter() {
        println!("{:?}", bc);
    }

    byte_code = optimizer::optimize(&byte_code);
    println!("Optimized byte code ({} ops):", byte_code.len());
    for bc in byte_code.iter() {
        println!("{:?}", bc);
    }

    let mut vm = ShrekVM::new(byte_code);
    let exit_code = match vm.run() {
        Ok(x) => x,
        Err(x) => {
            println!("Error: {}", x.message);
            -1
        }
    };

    std::process::exit(exit_code);
}

fn read_file(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}
