use crate::byte_code::{ByteCode, OpCode};

use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Whitespace,
    Command,
    Label,
    Comment
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub index: usize,
    pub value: String
}

pub struct Tokenizer {
    label_re: Regex,
    cmd_re: Regex,
    whitespace_regex: Regex,
    comment_regex: Regex
}

pub struct SyntaxNode {
    pub token: Token,
    pub children: Vec<SyntaxNode> // Next pointer?
}

pub struct SyntaxTree {
    pub tree: Vec<SyntaxNode>
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub index: usize,
    pub message: String
}

pub type ParseResult<T> = Result<T, SyntaxError>;

pub fn generate_byte_code(syntax_tree: &SyntaxTree) -> ParseResult<Vec<ByteCode>> {
    let mut byte_code = Vec::new();

    let mut label_map = HashMap::<&String, i32>::new();

    for node in syntax_tree.tree.iter() {
        match node.token.token_type {
            TokenType::Label => {
                // Get or generate a label number for this label.
                let arg = get_label_num(&mut label_map, &node.token.value);
                let code = ByteCode{ op_code: OpCode::Label, arg: arg };
                byte_code.push(code);
            },
            TokenType::Command => {
                let op_code = get_op_code(&node.token.value)
                    .ok_or_else(|| SyntaxError::new(node.token.index, "invalid command"))?;
                
                let mut code = ByteCode{ op_code, arg: 0};

                // Jumps will use the label's number as the argument.
                if code.op_code == OpCode::Jump {
                    // This is assumed to be checked in the parser. Reasserting this assumption here.
                    if node.children.is_empty() || node.children[0].token.token_type != TokenType::Label {
                        return Err(SyntaxError::new(node.token.index, "jump must be followed by a label"));
                    }
                    
                    // Get or generate a label number for this label.
                    let child_label = &node.children[0].token.value;
                    code.arg = get_label_num(&mut label_map, child_label);
                }

                byte_code.push(code);
            },
            _ => () // Evertying else does not get byte code.
        }
    };

    Ok(byte_code)
}

fn get_label_num<'a>(label_map: &mut HashMap<&'a String, i32>, label: &'a String) -> i32 {
    let arg = match label_map.get(&label) {
        Some(x) => *x,
        None => {
            let new_val: i32 = label_map.len() as i32;
            label_map.insert(&label, new_val);
            new_val
        }
    };
    arg
}

fn get_op_code(value: &str) -> Option<OpCode> {
    match value {
        "S" => Some(OpCode::Push0),
        "H" => Some(OpCode::Pop),
        "R" => Some(OpCode::Bump),
        "E" => Some(OpCode::Func),
        "K" => Some(OpCode::Jump),
        _ => None
    }
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            label_re: Regex::new(r"^![SHREK]+!").unwrap(),
            cmd_re: Regex::new(r"^[SHREK]").unwrap(),
            whitespace_regex: Regex::new(r"^\s+").unwrap(),
            comment_regex: Regex::new(r"^#[^\n]*\n?").unwrap()
        }
    }

    pub fn tokenize(&self, code: &str) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();

        let mut index: usize = 0;
        while index < code.len() {
            let token = self.next_token(index, code)?;
            index += token.value.len();
            tokens.push(token);
        };

        Ok(tokens)
    }

    fn next_token(&self, index: usize, code: &str) -> ParseResult<Token> {
        let mtch: regex::Match;
        let token_type: TokenType;
    
        let code_slice = &code[index..];

        if let Some(m) = self.label_re.find(code_slice) {
            token_type = TokenType::Label;
            mtch = m;
        } else if let Some(m) = self.cmd_re.find(code_slice) {
            token_type = TokenType::Command;
            mtch = m;
        } else if let Some(m) = self.whitespace_regex.find(code_slice) {
            token_type = TokenType::Whitespace;
            mtch = m;
        } else if let Some(m) = self.comment_regex.find(code_slice) {
            token_type = TokenType::Comment;
            mtch = m;
        } else {
            return Err(SyntaxError::new(index, "Invalid Token"));
        };
    
        Ok(Token {
            token_type,
            index,
            value: code_slice[mtch.start()..mtch.end()].to_string()
        })
    }
}

impl SyntaxTree {
    pub fn generate(tokens: &Vec<Token>) -> ParseResult<SyntaxTree> {
        let mut tree = SyntaxTree{ tree: Vec::new() };

        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            match token.token_type {
                TokenType::Command => {
                    tree.tree.push(SyntaxTree::parse_command(tokens, &mut index)?);
                },
                TokenType::Label => {
                    tree.tree.push(SyntaxTree::parse_label(tokens, &mut index)?);
                },
                // All other tokens are ignored (whitespace, comments)
                _ => { index += 1; }
            };
        };

        Ok(tree)
    }

    fn parse_command(tokens: &Vec<Token>, index: &mut usize) -> ParseResult<SyntaxNode> {
        // Assumes that index range check was done in caller.
        debug_assert!(*index < tokens.len());

        let token = &tokens[*index];
        *index += 1;

        let op_code = get_op_code(&token.value)
            .ok_or_else(|| SyntaxError::new(token.index, "invalid operation code"))?;

        let mut node = SyntaxNode { token: token.clone(), children: Vec::new() };

        // Jumps must be followed by a label. Enforce that rule here.
        if let OpCode::Jump = op_code {
            if *index < tokens.len() {
                // Inspect the next node and ensure it is a label. Make it a child of the node being processed.
                let next_token = &tokens[*index];
                if let TokenType::Label = next_token.token_type {
                    node.children.push(SyntaxTree::parse_label(tokens, index)?);
                } else {
                    return Err(SyntaxError::new(token.index, "missing label after jump"));
                }
            } else {
                return Err(SyntaxError::new(token.index, "missing command after jump"));
            };
        };
        
        Ok(node)
    }

    fn parse_label(tokens: &Vec<Token>, index: &mut usize) -> ParseResult<SyntaxNode> {
        // Assumes that index range check was done in caller.
        debug_assert!(*index < tokens.len());

        let token = &tokens[*index];
        *index += 1;

        let node = SyntaxNode { token: token.clone(), children: Vec::new() };
        Ok(node)
    }
}

impl SyntaxError {
    fn new(index: usize, message: &str) -> SyntaxError {
        SyntaxError {index, message: message.to_string()}
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error at index {}: {}", self.index, self.message)
    }
}
