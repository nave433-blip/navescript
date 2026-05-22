package lexer

import "testing"

func TestNextToken(t *testing.T) {
	input := `let five = 5;
	fn add(x, y) {
		x + y
	};
	if (5 < 10) {
		return true;
	} else {
		return false;
	}`

	tests := []struct {
		expectedType    TokenType
		expectedLiteral string
	}{
		{LET, "let"}, {IDENT, "five"}, {ASSIGN, "="}, {INT, "5"}, {SEMICOLON, ";"},
		{FUNCTION, "fn"}, {IDENT, "add"}, {LPAREN, "("}, {IDENT, "x"}, {COMMA, ","}, {IDENT, "y"}, {RPAREN, ")"}, {LBRACE, "{"},
		{IDENT, "x"}, {PLUS, "+"}, {IDENT, "y"}, {RBRACE, "}"}, {SEMICOLON, ";"},
		{IF, "if"}, {LPAREN, "("}, {INT, "5"}, {LT, "<"}, {INT, "10"}, {RPAREN, ")"}, {LBRACE, "{"},
		{RETURN, "return"}, {TRUE, "true"}, {SEMICOLON, ";"}, {RBRACE, "}"},
		{ELSE, "else"}, {LBRACE, "{"}, {RETURN, "return"}, {FALSE, "false"}, {SEMICOLON, ";"}, {RBRACE, "}"},
		{EOF, ""},
	}

	l := New(input)

	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%q, got=%q", i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q", i, tt.expectedLiteral, tok.Literal)
		}
	}
}
