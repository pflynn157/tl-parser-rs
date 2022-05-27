use std::fs::File;
use std::io::{BufRead, BufReader};

//
// Defines the tokens
//
#[derive(PartialEq, Debug)]
pub enum Token {
    None,
    Eof,
    
    // Keywords
    Func,
    Is,
    End,
    
    // Symbols
    LParen,
    RParen,
    SemiColon,
    
    // Literals
    Id(String),
    StringL(String),
}

//
// Defines a lexer context
//
pub struct Scanner {
    pub file_name : String,
    contents : String,
    pos : usize,
    buffer : String,
    stack : Vec<Token>,
}

impl Scanner {
    pub fn debug(&self) {
        println!("FILE: {}", self.file_name);
        println!("Contents: ");
        println!("{}", self.contents);
        println!("");
    }
    
    // Inits the lexer and loads a file
    pub fn init(&mut self) {
        match File::open(self.file_name.clone()) {
            Ok(file) => {
                let reader = BufReader::new(&file);

                for ln in reader.lines() {
                    let line = ln.unwrap().trim().to_string();
                    if line.len() > 0 {
                        self.contents += &line;
                        self.contents += &" ".to_string();
                    }
                }
            }

            Err(e) => {
                println!("Fatal Error!");
                println!("{}", e);
            }
        };
    }
    
    // Gets the next token in sequence
    pub fn get_next(&mut self) -> Token {
        loop {
            if self.stack.len() > 0 {
                return self.stack.pop().unwrap();
            }
        
            if self.pos >= self.contents.len() {
                return Token::Eof;
            }
            
            let mut c : char = self.contents.chars().nth(self.pos).unwrap();
            self.pos += 1;
            
            // Check string literals
            if c == '\"' {
                let mut val = String::new();
                c = self.contents.chars().nth(self.pos).unwrap();
                self.pos += 1;
                while c != '\"' {
                    val.push(c);
                    c = self.contents.chars().nth(self.pos).unwrap();
                    self.pos += 1;
                }
                return Token::StringL(val);
            }
            
            if self.is_separator(c) || self.is_symbol(c) {
                // If we have a symbol, get it and check the buffer
                if self.is_symbol(c) {
                    let token1 = self.get_symbol(c);
                    if self.buffer.len() == 0 {
                        return token1;
                    } else {
                        self.stack.push(token1);
                    }
                }
                
                // If the buffer is empty, do nothing
                if self.buffer.len() == 0 {
                    continue;
                }
                
                // Otherwise, keep going
                let mut token : Token = self.get_keyword();
                if token != Token::None {
                    self.buffer = String::new();
                    return token;
                }
                
                token = Token::Id(self.buffer.clone());
                self.buffer = String::new();
                return token;
            } else {
                self.buffer.push(c);
            }
        }
    }
    
    // A helper function for indicating whether we have whitespace
    fn is_separator(&self, c : char) -> bool {
        match c {
            ' ' | '\n' => return true,
            _ => return false,
        }
    }
    
    // A helper function for indicating whether we have a symbol
    fn is_symbol(&self, c : char) -> bool {
        match c {
              '('
            | ')'
            | ';' => return true,
            _ => return false,
        }
    }
    
    // A helper function for return a token based on a symbol
    fn get_symbol(&self, c : char) -> Token {
        match c {
            '(' => return Token::LParen,
            ')' => return Token::RParen,
            ';' => return Token::SemiColon,
            
            _ => return Token::None,
        }
    }
    
    // A helper function for converting the buffer to a token
    fn get_keyword(&self) -> Token {
        if self.buffer == "func" { return Token::Func; }
        else if self.buffer == "is" { return Token::Is; }
        else if self.buffer == "end" { return Token::End; }
        Token::None
    }
}

//
// A helper function for creating a lexer object
//
pub fn lex_new(file_name : String) -> Scanner {
    Scanner {
        file_name : file_name,
        contents : String::new(),
        pos : 0,
        buffer : String::new(),
        stack : Vec::new(),
    }
}

