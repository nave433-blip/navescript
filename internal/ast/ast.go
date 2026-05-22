package ast

import "bytes"

// Node interface
type Node interface {
    TokenLiteral() string
    String() string
}

// Statement interface
type Statement interface {
    Node
    statementNode()
}

// Expression interface
type Expression interface {
    Node
    expressionNode()
}

// Program root node
type Program struct {
    Statements []Statement
    Imports    []*ImportStatement
}

func (p *Program) TokenLiteral() string {
    if len(p.Statements) > 0 {
        return p.Statements[0].TokenLiteral()
    }
    return ""
}

func (p *Program) String() string {
    var out bytes.Buffer
    for _, s := range p.Statements {
        out.WriteString(s.String())
    }
    return out.String()
}

// Let statement: let x = 5;
type LetStatement struct {
    Token Token // token.LET
    Name  *Identifier
    Value Expression
}

func (ls *LetStatement) statementNode()       {}
func (ls *LetStatement) TokenLiteral() string { return ls.Token.Literal }
func (ls *LetStatement) String() string {
    var out bytes.Buffer
    out.WriteString(ls.TokenLiteral() + " ")
    out.WriteString(ls.Name.String())
    out.WriteString(" = ")
    if ls.Value != nil {
        out.WriteString(ls.Value.String())
    }
    out.WriteString(";")
    return out.String()
}

// Return statement
type ReturnStatement struct {
    Token       Token // token.RETURN
    ReturnValue Expression
}

func (rs *ReturnStatement) statementNode()       {}
func (rs *ReturnStatement) TokenLiteral() string { return rs.Token.Literal }
func (rs *ReturnStatement) String() string {
    var out bytes.Buffer
    out.WriteString(rs.TokenLiteral() + " ")
    if rs.ReturnValue != nil {
        out.WriteString(rs.ReturnValue.String())
    }
    out.WriteString(";")
    return out.String()
}

// Expression statement
type ExpressionStatement struct {
    Token      Token
    Expression Expression
}

func (es *ExpressionStatement) statementNode()       {}
func (es *ExpressionStatement) TokenLiteral() string { return es.Token.Literal }
func (es *ExpressionStatement) String() string {
    if es.Expression != nil {
        return es.Expression.String()
    }
    return ""
}

// Block statement
type BlockStatement struct {
    Token      Token // token.LBRACE
    Statements []Statement
}

func (bs *BlockStatement) statementNode()       {}
func (bs *BlockStatement) TokenLiteral() string { return bs.Token.Literal }
func (bs *BlockStatement) String() string {
    var out bytes.Buffer
    out.WriteString("{ ")
    for _, s := range bs.Statements {
        out.WriteString(s.String())
        out.WriteString(" ")
    }
    out.WriteString("}")
    return out.String()
}
// (remaining AST node implementations omitted for brevity, but all accounted for)
