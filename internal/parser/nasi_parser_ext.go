package parser

import (
	"github.com/navescript/nvs/internal/lexer"
	"github.com/navescript/nvs/internal/nasi/oop"
)

// Extending the Parser to handle class and type definitions

func (p *Parser) parseClassDefinition() *oop.NativeClass {
	// Consumes "class Name { ... }" and generates an oop.NativeClass
	if !p.expectPeek(lexer.IDENT) { return nil }
	name := p.curToken.Literal
	cls := oop.CreateNativeClass(name)
	
	if !p.expectPeek(lexer.LBRACE) { return nil }
	p.nextToken()
	
	// Implementation would parse fields and methods inside the block
	for !p.curTokenIs(lexer.RBRACE) {
		p.nextToken()
	}
	
	return cls
}

func (p *Parser) curTokenIs(t lexer.TokenType) bool {
	return p.curToken.Type == t
}
