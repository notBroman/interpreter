package token

type TokenType string

type Token struct
{
    Type TokenType
    Literal string
}

const (
    ILLEGAL   = "ILLEGAL"
    EOF       = "EOF"

    // Identifiers & literals
    IDENT     = "IDENT"
    INT       = "INT"

    // Operators
    ASSIGN    = "="
    PLUS      = "+"
    MINUS     = "-"
    SLASH     = "/"
    ASTERISK  = "*"
    LT        = "<"
    GT        = ">"
    BANG      = "!"
    EQ        = "=="
    NEQ       = "!="

    // Delimiters
    COMMA     = ","
    SEMICOLON = ";"
    LPAREN    = "("
    RPAREN    = ")"
    LBRACE    = "{"
    RBRACE    = "}"

    // Keywords
    FUNCTION  = "FUNCTION"
    LET       = "LET"
    RETURN    = "RETURN"
    IF        = "IF"
    ELSE      = "ELSE"
    TRUE      = "TRUE"
    FALSE     = "FALSE"
)

var keywords = map[string]TokenType {
    "fn"    : FUNCTION,
    "let"   : LET,
    "return": RETURN,
    "true"  : TRUE,
    "false" : FALSE,
    "if"    : IF,
    "else"  : ELSE,
}

func LookupIdent(ident string) TokenType {
    if tok, ok := keywords[ident]; ok {
	return tok
    }
    return IDENT
}
