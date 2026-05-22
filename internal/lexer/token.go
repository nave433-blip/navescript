package lexer

type TokenType string

const (
    // Special
    ILLEGAL TokenType = "ILLEGAL"
    EOF     TokenType = "EOF"

    // Identifiers & literals
    IDENT  TokenType = "IDENT"
    INT    TokenType = "INT"
    FLOAT  TokenType = "FLOAT"
    STRING TokenType = "STRING"
    BOOL   TokenType = "BOOL"

    // Operators
    ASSIGN   TokenType = "="
    PLUS     TokenType = "+"
    MINUS    TokenType = "-"
    BANG     TokenType = "!"
    ASTERISK TokenType = "*"
    SLASH    TokenType = "/"
    MOD      TokenType = "%"
    LT       TokenType = "<"
    GT       TokenType = ">"
    EQ       TokenType = "=="
    NOT_EQ   TokenType = "!="
    AND      TokenType = "&&"
    OR       TokenType = "||"

    // Delimiters
    COMMA     TokenType = ","
    SEMICOLON TokenType = ";"
    LPAREN    TokenType = "("
    RPAREN    TokenType = ")"
    LBRACE    TokenType = "{"
    RBRACE    TokenType = "}"
    LBRACKET  TokenType = "["
    RBRACKET  TokenType = "]"

    // Keywords
    FUNCTION TokenType = "FUNCTION"
    LET      TokenType = "LET"
    TRUE     TokenType = "TRUE"
    FALSE    TokenType = "FALSE"
    IF       TokenType = "IF"
    ELSE     TokenType = "ELSE"
    RETURN   TokenType = "RETURN"
    WHILE    TokenType = "WHILE"
    FOR      TokenType = "FOR"
    IMPORT   TokenType = "IMPORT"
)

type Token struct {
    Type    TokenType
    Literal string
    Line    int
    Column  int
}
