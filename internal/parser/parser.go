package parser

import (
	"fmt"
	"github.com/navescript/nvs/internal/lexer"
)

// ParseProgram orchestrates the AST construction with robust error handling
func (p *Parser) ParseProgram() (*Program, error) {
	program := &Program{
		Statements: []Statement{},
		Imports:    []*ImportStatement{},
	}

	for p.curToken.Type != lexer.EOF {
		stmt := p.parseStatement()
		if stmt != nil {
			if importStmt, ok := stmt.(*ImportStatement); ok {
				program.Imports = append(program.Imports, importStmt)
			} else {
				program.Statements = append(program.Statements, stmt)
			}
		}
		p.nextToken()
	}

	if len(p.errors) > 0 {
		return program, fmt.Errorf("parsing failed with %d errors: %v", len(p.errors), p.errors)
	}

	return program, nil
}
