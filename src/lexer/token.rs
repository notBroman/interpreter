#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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

// const TOKEN_LITERAL: HashMap<TokenType, String> = [
//     (TokenType::ILLEGAL, "ILLEGAL"),
//     (TokenType::EOF, "EOF"),
//     (TokenType::IDENT, "IDENT"),
//     (TokenType::INT, "INT"),
//     (TokenType::ASSIGN, "="),
//     (TokenType::PLUS, "+"),
//     (TokenType::MINUS, "-"),
//     (TokenType::SLASH, "/"),
//     (TokenType::ASTERISK, "*"),
//     (TokenType::LT, "<"),
//     (TokenType::GT, ">"),
//     (TokenType::BANG, "!"),
//     (TokenType::EQ, "=="),
//     (TokenType::NEQ, "!="),
//     (TokenType::COMMA, ","),
//     (TokenType::SEMICOLON, ";"),
//     (TokenType::LPAREN, "("),
//     (TokenType::RPAREN, ")"),
//     (TokenType::LBRACE, "{"),
//     (TokenType::RBRACE, "}"),
//     (TokenType::FUNCTION, "FUNCTION"),
//     (TokenType::LET, "LET"),
//     (TokenType::RETURN, "RETURN"),
//     (TokenType::IF, "IF"),
//     (TokenType::ELSE, "ELSE"),
//     (TokenType::TRUE, "TRUE"),
//     (TokenType::FALSE, "FALSE"),
// ];

pub struct Token {
    pub tokentype: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String) -> Self {
        Self {
            tokentype: ttype,
            literal: literal,
        }
    }
}
