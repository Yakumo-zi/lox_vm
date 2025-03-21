use crate::{
    chunk::{
        Chunk,
        OpCode::{self, *},
    },
    common::Value,
    scanner::{Scanner, Token, TokenType},
};
use anyhow::{Result, anyhow};
use std::io::Write;
use std::io::stderr;
enum Precedence{
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}
#[derive(Debug, Default)]
struct Parser {
    current: Option<Token>,
    prev: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}
impl Parser {
    fn error(&mut self, msg: &str) {
        let tok = self.prev.take();
        if let Some(tok) = &tok {
            self.error_at(tok, msg);
        }
        self.prev = tok;
    }
    fn error_at_current(&mut self, msg: &str) {
        let tok = self.current.take();
        if let Some(tok) = &tok {
            self.error_at(tok, msg);
        }
        self.current = tok;
    }
    fn error_at(&mut self, tok: &Token, msg: &str) {
        if self.panic_mode {
            return;
        }
        write!(stderr(), "[line {}] Error", tok.line).unwrap();
        if tok.typ == TokenType::EOF {
            write!(stderr(), " at end").unwrap();
        } else if tok.typ == TokenType::Error {
        } else {
            write!(stderr(), " at '{}'", &tok.literal).unwrap();
        }
        write!(stderr(), ": {}\n", msg).unwrap();
        self.had_error = true;
    }
}
pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    chunk: Chunk,
}

impl Compiler {
    pub fn new(source: &str) -> Compiler {
        Compiler {
            scanner: Scanner::new(source),
            parser: Parser::default(),
            chunk: Chunk::default(),
        }
    }
    pub fn compile(&mut self) -> Result<Chunk> {
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.");
        if self.parser.had_error {
            return Err(anyhow!("Compile error"));
        }
        self.chunk.write(Return, self.scanner.line);
        Ok(Chunk::default())
    }
    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }
    fn number(&mut self) {
        if let Some(tok) = &self.parser.prev {
            if let Ok(value) = tok.literal.parse::<f64>() {
                let constant_idx = self.chunk.add_constant(value);
                self.chunk.write(Constant(constant_idx), tok.line);
            }
        }
    }
    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }
    fn unary(&mut self) {
        let tok=self.parser.prev.take();
        if let Some(tok) = &tok {
            let tok_type = tok.typ.clone();
            self.parse_precedence(Precedence::Unary);
            match tok_type {
                TokenType::Minus => {
                    self.chunk.write(OpCode::Negate, tok.line);
                }
                _ => return,
            }
        }
    }
    fn binary(&mut self){
        
    }
    fn parse_precedence(&mut self,precedence:Precedence){}
    fn advance(&mut self) {
        self.parser.prev = self.parser.current.take();
        loop {
            let token = self.scanner.scan_token();
            if let Some(tok) = &token {
                if tok.typ != TokenType::Error {
                    self.parser.current = token;
                    break;
                }
                self.parser.error_at_current(&tok.literal);
            }
            self.parser.current = token;
        }
    }
    fn consume(&mut self, typ: TokenType, msg: &str) {
        if let Some(tok) = &self.parser.current {
            if tok.typ == typ {
                self.advance();
                return;
            }
        }
        self.parser.error_at_current(msg);
    }
}
