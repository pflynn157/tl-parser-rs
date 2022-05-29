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
    Return,
    Var,
    While,
    Do,
    If, Elif, Else,
    Then,
    Break, Continue,
    Struct,
    
    // Type keywords
    I8, U8,
    I16, U16,
    I32, U32,
    I64, U64,
    String,
    Char,
    Bool,
    
    // Symbols
    LParen, RParen,
    LBracket, RBracket,
    SemiColon,
    Colon,
    Comma,
    Dot,
    Arrow,
    Assign,
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor,
    Eq, Ne, Gt, Lt, Ge, Le,
    LGAnd, LGOr,
    
    // Literals
    Id(String),
    StringL(String),
    IntL(u64),
    CharL(char),
    True, False,
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
    
    // Unget the last token
    pub fn unget(&mut self, token : Token) {
        self.stack.push(token);
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
            
            let mut c = self.get_char();
            
            // Check string literals
            if c == '\"' {
                let mut val = String::new();
                c = self.get_char();
                while c != '\"' {
                    val.push(c);
                    c = self.get_char();
                }
                return Token::StringL(val);
            }
            
            // Check character literals
            if c == '\'' {
                let c2 = self.get_char();
                self.get_char();        // Assume '
                return Token::CharL(c2);
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
                
                // See if we have an integer
                if self.is_integer() {
                    token = Token::IntL(self.get_integer());
                    self.buffer = String::new();
                    return token;
                }
                
                // Otherwise, we have an indentifier
                token = Token::Id(self.buffer.clone());
                self.buffer = String::new();
                return token;
            } else {
                self.buffer.push(c);
            }
        }
    }
    
    // A helper function for getting the next character in the stream
    fn get_char(&mut self) -> char {
        let c : char = self.contents.chars().nth(self.pos).unwrap();
        self.pos += 1;
        c
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
            | '[' | ']'
            | ';'
            | ':'
            | ',' 
            | '.'
            | '+' | '-' | '*' | '/' | '%' 
            | '&' | '|' | '^' 
            | '=' | '!' | '>' | '<' => return true,
            _ => return false,
        }
    }
    
    // A helper function for seeing if buffer is an integer
    fn is_integer(&self) -> bool {
        let num : Result<u64, _> = self.buffer.trim().parse();
        match num {
            Ok(_) => return true,
            _ => return false,
        }
    }
    
    // A helper function for return a token based on a symbol
    fn get_symbol(&mut self, c : char) -> Token {
        match c {
            '(' => return Token::LParen,
            ')' => return Token::RParen,
            '[' => return Token::LBracket,
            ']' => return Token::RBracket,
            ';' => return Token::SemiColon,
            ',' => return Token::Comma,
            '.' => return Token::Dot,
            '+' => return Token::Add,
            '*' => return Token::Mul,
            '/' => return Token::Div,
            '%' => return Token::Mod,
            '^' => return Token::Xor,
            '=' => return Token::Eq,
            
            '-' => {
                let c2 = self.get_char();
                if c2 == '>' {
                    return Token::Arrow;
                }
                self.pos -= 1;
                return Token::Sub;
            },
            
            ':' => {
                let c2 = self.get_char();
                if c2 == '=' {
                    return Token::Assign;
                }
                self.pos -= 1;
                return Token::Colon;
            },
            
            '!' => {
                let c2 = self.get_char();
                if c2 == '=' {
                    return Token::Ne;
                }
                self.pos -= 1;
                return Token::None;
            },
            
            '>' => {
                let c2 = self.get_char();
                if c2 == '=' {
                    return Token::Ge;
                }
                self.pos -= 1;
                return Token::Gt;
            },
            
            '<' => {
                let c2 = self.get_char();
                if c2 == '=' {
                    return Token::Le;
                }
                self.pos -= 1;
                return Token::Lt;
            },
            
            '&' => {
                let c2 = self.get_char();
                if c2 == '&' {
                    return Token::LGAnd;
                }
                self.pos -= 1;
                return Token::And;
            },
            
            '|' => {
                let c2 = self.get_char();
                if c2 == '|' {
                    return Token::LGOr;
                }
                self.pos -= 1;
                return Token::Or;
            },
            
            _ => return Token::None,
        }
    }
    
    // A helper function for converting the buffer to a token
    fn get_keyword(&self) -> Token {
        if self.buffer == "func" { return Token::Func; }
        else if self.buffer == "is" { return Token::Is; }
        else if self.buffer == "end" { return Token::End; }
        else if self.buffer == "return" { return Token::Return; }
        else if self.buffer == "var" { return Token::Var; }
        else if self.buffer == "while" { return Token::While; }
        else if self.buffer == "do" { return Token::Do; }
        else if self.buffer == "if" { return Token::If; }
        else if self.buffer == "elif" { return Token::Elif; }
        else if self.buffer == "else" { return Token::Else; }
        else if self.buffer == "then" { return Token::Then; }
        else if self.buffer == "break" { return Token::Break; }
        else if self.buffer == "continue" { return Token::Continue; }
        else if self.buffer == "struct" { return Token::Struct; }
        else if self.buffer == "i8" { return Token::I8; }
        else if self.buffer == "u8" { return Token::U8; }
        else if self.buffer == "i16" { return Token::I16; }
        else if self.buffer == "u16" { return Token::U16; }
        else if self.buffer == "i32" { return Token::I32; }
        else if self.buffer == "u32" { return Token::U32; }
        else if self.buffer == "i64" { return Token::I64; }
        else if self.buffer == "u64" { return Token::U64; }
        else if self.buffer == "string" { return Token::String; }
        else if self.buffer == "char" { return Token::Char; }
        else if self.buffer == "bool" { return Token::Bool; }
        else if self.buffer == "true" { return Token::True; }
        else if self.buffer == "false" { return Token::False; }
        Token::None
    }
    
    // A helper function for converting a buffer into an integer
    fn get_integer(&self) -> u64 {
        let num : Result<u64, _> = self.buffer.trim().parse();
        match num {
            Ok(num) => return num,
            _ => return 0,
        }
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

