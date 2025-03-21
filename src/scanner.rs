use std::{future::pending, iter::Scan, ops::Not};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Special tokens
    Error,
    EOF,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
    pub line: i32,
}
impl Token {
    pub fn new(typ: TokenType, literal: String, line: i32) -> Token {
        Token {
            typ: typ,
            literal: literal,
            line: line,
        }
    }
}
pub struct Scanner {
    start: usize,
    current: usize,
    source: String,
    line: i32,
}
impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            source: source.into(),
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Option<Token> {
        self.skip_white_space();
        self.start = self.current;
        if self.is_at_end()? {
            return Some(self.make_token(TokenType::EOF));
        }
        let c = self.advance()?;
        if c.is_alphabetic() {
            return self.identifer();
        }
        if c.is_digit(10) {
            return self.number();
        }
        match c {
            '(' => Some(self.make_token(TokenType::LeftParen)),
            ')' => Some(self.make_token(TokenType::RightParen)),
            '{' => Some(self.make_token(TokenType::LeftBrace)),
            '}' => Some(self.make_token(TokenType::RightBrace)),
            ';' => Some(self.make_token(TokenType::Semicolon)),
            ',' => Some(self.make_token(TokenType::Comma)),
            '.' => Some(self.make_token(TokenType::Dot)),
            '-' => Some(self.make_token(TokenType::Minus)),
            '+' => Some(self.make_token(TokenType::Plus)),
            '/' => Some(self.make_token(TokenType::Slash)),
            '*' => Some(self.make_token(TokenType::Star)),
            '!' => {
                let mut typ = TokenType::Bang;
                if self.match_next('=')? {
                    typ = TokenType::BangEqual;
                }
                Some(self.make_token(typ))
            }
            '=' => {
                let mut typ = TokenType::Equal;
                if self.match_next('=')? {
                    typ = TokenType::EqualEqual;
                }
                Some(self.make_token(typ))
            }
            '<' => {
                let mut typ = TokenType::Less;
                if self.match_next('=')? {
                    typ = TokenType::LessEqual;
                }
                Some(self.make_token(typ))
            }
            '>' => {
                let mut typ = TokenType::Greater;
                if self.match_next('=')? {
                    typ = TokenType::GreaterEqual;
                }
                Some(self.make_token(typ))
            }
            '"' => self.string(),

            _ => Some(self.error_token("Unexpected character.")),
        }
    }
    fn skip_white_space(&mut self) {
        loop {
            let c = self.peek();
            match c {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') => {
                    if self.peek_next().unwrap() == '/' {
                        while self.peek().unwrap() != '\n' && !self.is_at_end().unwrap() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }
    fn identifer(&mut self) -> Option<Token> {
        while self.peek()?.is_alphanumeric() {
            self.advance()?;
        }
        Some(self.make_token(self.identifier_type()))
    }
    fn identifier_type(&self) -> TokenType {
        use TokenType::*;
        let c = self.source.chars().nth(self.start).unwrap();
        println!("identifier_char {}",c);
        match c {
            'a' => self.check_word(2, "and", And),
            'c' => self.check_word(4, "class", Class),
            'e' => self.check_word(3, "else", Else),
            'i' => self.check_word(1, "if", If),
            'n' => self.check_word(2, "nil", Nil),
            'o' => self.check_word(1, "or", Or),
            'p' => self.check_word(4, "print", Print),
            'r' => self.check_word(5, "return", Return),
            's' => self.check_word(4, "super", Super),
            'v' => self.check_word(2, "var", Var),
            'w' => self.check_word(4, "while", While),
            'f' => {
                if self.current - self.start > 1 {
                    let c = self.source.chars().nth(self.start+1).unwrap();
                    match c {
                        'a' => self.check_word(4, "false", False),
                        'o' => self.check_word(2, "for", For),
                        'u' => self.check_word(2, "fun", Fun),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    let c = self.source.chars().nth(self.start+1).unwrap();
                    match c {
                        'h' => self.check_word(3, "this", This),
                        'r' => self.check_word(3, "true", True),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            _ => Identifier,
        }
    }
    fn check_word(&self, len: usize, expect: &'static str, typ: TokenType) -> TokenType {
        let start_idx = self.source[..self.start].chars().count();
        let end_idx = self.source[..self.start+len+1].chars().count();
        let literal = self
            .source
            .chars()
            .skip(start_idx)
            .take(end_idx - start_idx)
            .collect::<String>();
        if literal.eq(expect) {
            typ
        } else {
            TokenType::Identifier
        }
    }
    fn number(&mut self) -> Option<Token> {
        while self.peek()?.is_digit(10) {
            self.advance()?;
        }
        if self.peek()? == '.' && self.peek_next()?.is_digit(10) {
            self.advance()?;
            while self.peek()?.is_digit(10) {
                self.advance()?;
            }
        }
        Some(self.make_token(TokenType::Number))
    }
    fn string(&mut self) -> Option<Token> {
        while self.peek()? != '"' && !self.is_at_end()? {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }
        if self.is_at_end()? {
            return Some(self.error_token("Unterminated string."));
        }
        self.advance()?;
        Some(self.make_token(TokenType::String))
    }
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn peek_next(&self) -> Option<char> {
        if self.is_at_end()? {
            return Some('\0');
        }
        self.source.chars().nth(self.current + 1)
    }
    fn make_token(&self, typ: TokenType) -> Token {
        let start_idx = self.source[..self.start].chars().count();
        let end_idx = self.source[..self.current].chars().count();
        let literal = self
            .source
            .chars()
            .skip(start_idx)
            .take(end_idx - start_idx)
            .collect::<String>();
        Token::new(typ, literal, self.line)
    }
    fn error_token(&self, msg: &str) -> Token {
        Token {
            typ: TokenType::Error,
            literal: msg.to_string(),
            line: self.line,
        }
    }
    fn is_at_end(&self) -> Option<bool> {
        let c = self.source.chars().nth(self.current)?;
        Some(c == '\0')
    }
    fn match_next(&mut self, expcted: char) -> Option<bool> {
        if self.is_at_end()? {
            return Some(false);
        }
        let c = self.source.chars().nth(self.current)?;
        if c != expcted {
            return Some(false);
        }
        self.current += 1;
        Some(true)
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        let c = self.source.chars().nth(self.current - 1)?;
        Some(c)
    }
}
