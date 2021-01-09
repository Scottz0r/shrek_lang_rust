mod shrek_parser;

use shrek_parser::Tokenizer;
//use regex::{Regex};

fn main() {
    // TODO: Read this from file.
    let test_data = "# Stuff\n!R!SRR # Test";


    let tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(test_data).unwrap(); // TODO: Error checking.

    for t in tokens.iter() {
        println!("Found {:?} token {:?} at {:?}", t.token_type, t.value, t.index);
    }

}
