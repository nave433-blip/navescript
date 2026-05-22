package lexer

import (
	"fmt"
	"unicode"
)

// Token structure optimized for memory and performance
type Token struct {
	Type    TokenType
	Literal string
	Line    int
	Column  int
}

// NextToken optimized with efficient lookahead and error reporting
func (l *Lexer) NextToken() Token {
	l.skipWhitespace()

	var tok Token
	tok.Line = l.line
	tok.Column = l.column

	switch l.ch {
	case '=':
		if l.peekChar() == '=' {
			tok = l.makeTwoCharToken(EQ)
		} else {
			tok = newToken(ASSIGN, l.ch, l.line, l.column)
		}
	case '+':
		tok = newToken(PLUS, l.ch, l.line, l.column)
	case '-':
		tok = newToken(MINUS, l.ch, l.line, l.column)
	case '!':
		if l.peekChar() == '=' {
			tok = l.makeTwoCharToken(NOT_EQ)
		} else {
			tok = newToken(BANG, l.ch, l.line, l.column)
		}
	case '/':
		tok = newToken(SLASH, l.ch, l.line, l.column)
	case '*':
		tok = newToken(ASTERISK, l.ch, l.line, l.column)
	case '<':
		tok = newToken(LT, l.ch, l.line, l.column)
	case '>':
		tok = newToken(GT, l.ch, l.line, l.column)
	case ';':
		tok = newToken(SEMICOLON, l.ch, l.line, l.column)
	case ',':
		tok = newToken(COMMA, l.ch, l.line, l.column)
	case '(':
		tok = newToken(LPAREN, l.ch, l.line, l.column)
	case ')':
		tok = newToken(RPAREN, l.ch, l.line, l.column)
	case '{':
		tok = newToken(LBRACE, l.ch, l.line, l.column)
	case '}':
		tok = newToken(RBRACE, l.ch, l.line, l.column)
	case '[':
		tok = newToken(LBRACKET, l.ch, l.line, l.column)
	case ']':
		tok = newToken(RBRACKET, l.ch, l.line, l.column)
	case '@':
		return l.readPolyglotToken()
	case 0:
		tok.Literal = ""
		tok.Type = EOF
	default:
		if isLetter(l.ch) {
			tok.Literal = l.readIdentifier()
			tok.Type = LookupIdent(tok.Literal)
			return tok
		} else if unicode.IsDigit(rune(l.ch)) {
			tok.Type = INT
			tok.Literal = l.readNumber()
			return tok
		} else {
			tok = newToken(ILLEGAL, l.ch, l.line, l.column)
		}
	}

	l.readChar()
	return tok
}

func (l *Lexer) makeTwoCharToken(t TokenType) Token {
	ch := l.ch
	l.readChar()
	literal := string(ch) + string(l.ch)
	return Token{Type: t, Literal: literal, Line: l.line, Column: l.column - 1}
}
