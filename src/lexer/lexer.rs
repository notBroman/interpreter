use crate::lexer::token;
use std::collections::HashMap;
use std::todo;
use std::vec;

use crate::lexer::token::TokenType;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,      // current char under inspection
    read_position: usize, // current char to be read (after current char)
    ch: u8,               // only ASCII supported
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let kwords: HashMap<String, TokenType> = HashMap::from([
            ("fn".to_string(), TokenType::FUNCTION),
            ("let".to_string(), TokenType::LET),
            ("return".to_string(), TokenType::RETURN),
            ("true".to_string(), TokenType::TRUE),
            ("false".to_string(), TokenType::FALSE),
            ("if".to_string(), TokenType::IF),
            ("else".to_string(), TokenType::ELSE),
        ]);
        let mut l: Self = Self {
            input: input.into_bytes(),
            position: 0,
            read_position: 1,
            ch: 0,
            keywords: kwords,
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch: u8 = self.ch;
                    self.read_char();
                    tok = self.new_token(TokenType::EQ, vec![ch, self.ch]);
                } else {
                    tok = self.new_token(TokenType::ASSIGN, vec![self.ch]);
                }
            }
            b';' => {
                tok = self.new_token(TokenType::SEMICOLON, vec![self.ch]);
            }
            b'(' => {
                tok = self.new_token(TokenType::LPAREN, vec![self.ch]);
            }
            b')' => {
                tok = self.new_token(TokenType::RPAREN, vec![self.ch]);
            }
            b',' => {
                tok = self.new_token(TokenType::COMMA, vec![self.ch]);
            }
            b'+' => {
                tok = self.new_token(TokenType::PLUS, vec![self.ch]);
            }
            b'-' => {
                tok = self.new_token(TokenType::MINUS, vec![self.ch]);
            }
            b'/' => {
                tok = self.new_token(TokenType::SLASH, vec![self.ch]);
            }
            b'*' => {
                tok = self.new_token(TokenType::ASTERISK, vec![self.ch]);
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch: u8 = self.ch;
                    self.read_char();
                    tok = self.new_token(TokenType::NEQ, vec![ch, self.ch]);
                } else {
                    tok = self.new_token(TokenType::BANG, vec![self.ch]);
                }
            }
            b'{' => {
                tok = self.new_token(TokenType::LBRACE, vec![self.ch]);
            }
            b'}' => {
                tok = self.new_token(TokenType::RBRACE, vec![self.ch]);
            }
            b'<' => {
                tok = self.new_token(TokenType::LT, vec![self.ch]);
            }
            b'>' => {
                tok = self.new_token(TokenType::GT, vec![self.ch]);
            }
            0 => {
                tok = self.new_token(TokenType::EOF, vec![b'0']);
            }
            _ => {
                todo!();
                // if string -> IDENT
                // if digits -> INT
            }
        }
        self.read_char();
        return tok;
    }

    pub fn lookup_ident(&self, ident: String) -> TokenType {
        let tok = self.keywords.get(&ident);
        match tok {
            Some(tok) => return *tok,
            None => return TokenType::IDENT,
        }
    }

    fn new_token(&mut self, ttype: TokenType, bytes: Vec<u8>) -> token::Token {
        token::Token::new(ttype, String::from_utf8(bytes).unwrap())
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn is_letter(&self, ch: u8) -> bool {
        return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
    }

    fn is_digit(&self, ch: u8) -> bool {
        return b'0' <= ch && ch <= b'9';
    }

    fn read_number(&mut self) -> String {
        let position: usize = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        return String::from_utf8(self.input[position..=self.position].to_vec()).unwrap();
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\r' || self.ch == b'\n' {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }
}
