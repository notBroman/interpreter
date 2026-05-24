use std::collections::HashMap;

pub enum TokenType {
    ILLEGAL = 0,
    EOF,
    // Idetifier & literals
    IDENT,
    INT,
    //Operators
    ASSIGN,
    PLUS,
    MINUS,
    SLASH,
    ASTERISK,
    LT,
    GT,
    BANG,
    EQ,
    NEQ,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    RETURN,
    IF,
    ELSE,
    TRUE,
    FALSE,
}

const enumLiteral: HashMap<TokenType, String> = {
    ILLEGAL = "ILLEGAL";
    EOF = "EOF";
    IDENT = "IDENT";
    INT = "INT";
    ASSIGN = "=";
    PLUS = "+";
    MINUS = "-";
    SLASH = "/";
    ASTERISK = "*";
    LT = "<";
    GT = ">";
    BANG = "!";
    EQ = "==";
    NEQ = "!=";
    COMMA = ",";
    SEMICOLON = ";";
    LPAREN = "(";
    RPAREN = ")";
    LBRACE = "{";
    RBRACE = "}";
    FUNCTION = "FUNCTION";
    LET = "LET";
    RETURN = "RETURN";
    IF = "IF";
    ELSE = "ELSE";
    TRUE = "TRUE";
    FALSE = "FALSE";
};

pub struct Token {
    Type: TokenType,
    Literal: String,
}

impl Token {
    pub fn New(Type: TokenType, Literal: String) -> Self {
        Self {
            Type: TokenType,
            Literal: Literal,
        }
    }

    pub fn getTokenTypeName() -> String {
        enumLiteral.get(Self.Type);
    }
}

pub fn LookupIdent(ident: String) -> TokenType {
    match keywords.get(ident) {
        Some(tok) => return tok,
        None => return crate::TokenType::IDENT,
    }
}

pub const keywords: HashMap<String, TokenType> = {
    "fn" = FUNCTION;
    "let" = LET;
    "return" = RETURN;
    "true" = TRUE;
    "false" = FALSE;
    "if" = IF;
    "else" = ELSE;
};
