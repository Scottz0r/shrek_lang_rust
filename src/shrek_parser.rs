use std::fmt;
use std::vec::Vec;
use regex::Regex;

#[derive(Debug)]
pub enum TokenType {
    Whitespace,
    Command,
    Label,
    Comment
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub index: usize,
    pub value: &'a str
}

pub struct Tokenizer {
    label_re: Regex,
    cmd_re: Regex,
    whitespace_regex: Regex,
    comment_regex: Regex
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub index: usize,
    pub message: String
}

pub type ParseResult<T> = Result<T, SyntaxError>;

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            label_re: Regex::new(r"^![SHREK]+!").unwrap(),
            cmd_re: Regex::new(r"^[SHREK]").unwrap(),
            whitespace_regex: Regex::new(r"^\s+").unwrap(),
            comment_regex: Regex::new(r"^#[^\n]*\n?").unwrap()
        }
    }

    pub fn tokenize<'a>(&self, code: &'a str) -> ParseResult<Vec<Token<'a>>> {
        let mut tokens = Vec::new();

        let mut index: usize = 0;
        while index < code.len() {
            let token = self.next_token(index, code)?;
            index += token.value.len();
            tokens.push(token);
        };

        Ok(tokens)
    }

    fn next_token<'a>(&self, index: usize, code: &'a str) -> ParseResult<Token<'a>> {
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
            value: &code_slice[mtch.start()..mtch.end()]
        })
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
