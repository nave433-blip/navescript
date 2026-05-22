// navescript/src/ns_parser.rs
use crate::ns_lexer::{Token, TokenType, Literal};
use crate::types::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Variable(String),
    Range(Literal, Literal, bool), // start, end, inclusive
    Array(Vec<Pattern>, Option<Box<Pattern>>), // items, rest
    Object(HashMap<String, Pattern>, Option<String>), // fields, rest_name
    Union(Vec<Pattern>),
    Wildcard,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    This,
    Super { method: Token },
    Binary { left: Box<Expr>, op: Token, right: Box<Expr> },
    Logical { left: Box<Expr>, op: Token, right: Box<Expr> },
    Unary { op: Token, right: Box<Expr> },
    Assignment { target: Box<Expr>, value: Box<Expr> },
    CompoundAssignment { target: Box<Expr>, op: Token, value: Box<Expr> },
    Ternary { cond: Box<Expr>, then: Box<Expr>, else_: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    New { class_name: String, args: Vec<Expr> },
    Await(Box<Expr>),
    Array(Vec<Expr>),
    Object(HashMap<String, Expr>),
    Grouping(Box<Expr>),
    Match { cond: Box<Expr>, arms: Vec<(Pattern, Option<Expr>, Expr)> },
    Get { object: Box<Expr>, name: Token },
    Set { object: Box<Expr>, name: Token, value: Box<Expr> },
    Greek(GreekExpr),
    CustomBinary { left: Box<Expr>, op: Token, right: Box<Expr> },
    UnitConversion { expr: Box<Expr>, target_unit: String },
    Infer(String),
}

#[derive(Debug, Clone)]
pub enum GreekExpr {
    QuantumSuperposition { expr: Box<Expr> },
    QuantumEntangle { left: Box<Expr>, right: Box<Expr> },
    QuantumMeasure { expr: Box<Expr> },
    QuantumGate { gate: QuantumGate, qubit: Box<Expr> },
    FourierTransform { expr: Box<Expr>, inverse: bool },
    Convolution { left: Box<Expr>, right: Box<Expr> },
    Correlation { left: Box<Expr>, right: Box<Expr> },
    Derivative { expr: Box<Expr>, at: Option<Box<Expr>> },
    Integral { expr: Box<Expr>, from: Option<Box<Expr>>, to: Option<Box<Expr>> },
    Limit { expr: Box<Expr>, at: Box<Expr>, direction: LimitDirection },
    SetUnion { left: Box<Expr>, right: Box<Expr> },
    SetIntersection { left: Box<Expr>, right: Box<Expr> },
    SetComplement { expr: Box<Expr> },
    BetaReduce { expr: Box<Expr> },
    EtaReduce { expr: Box<Expr> },
    TimeSeries { expr: Box<Expr> },
    Delay { ms: Box<Expr> },
    PhaseShift { expr: Box<Expr>, angle: Box<Expr> },
    PhaseRotate { expr: Box<Expr>, phase: Box<Expr> },
    RandomSample { dist: Distribution },
    MonteCarlo { samples: Box<Expr>, func: Box<Expr> },
    BrownianMotion { time: Box<Expr> },
    AsyncStart { expr: Box<Expr> },
    EventHandler { event: Box<Expr>, handler: Box<Expr> },
    Observable { producer: Box<Expr> },
    Immutable { expr: Box<Expr> },
    Unsafe { expr: Box<Expr> },
    WeakRef { expr: Box<Expr> },
    HeapAlloc { size: Box<Expr> },
    ObjectRef { expr: Box<Expr> },
    WhileLoop { cond: Box<Expr>, body: Vec<Stmt> },
    ForLoop { init: Box<Expr>, cond: Box<Expr>, inc: Box<Expr>, body: Vec<Stmt> },
    Omega { op: OmegaOp, body: Vec<Stmt> },
}

#[derive(Debug, Clone)]
pub enum OmegaOp { Transcend, RealityCheck, Bootstrap, VeilLifted }

#[derive(Debug, Clone)]
pub enum QuantumGate {
    Hadamard, PauliX, PauliY, PauliZ, CNOT, SWAP, Toffoli, Fredkin,
    Phase { angle: f64 }, RotationX { angle: f64 }, RotationY { angle: f64 }, RotationZ { angle: f64 },
    SGate, TGate, Sdg, Tdg,
}

#[derive(Debug, Clone)]
pub enum LimitDirection { FromAbove, FromBelow, Both }

#[derive(Debug, Clone)]
pub enum Distribution { Normal, Uniform, Poisson, Markov, MonteCarlo }

#[derive(Debug, Clone)]
pub struct Decorator {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct TypeParam {
    pub name: String,
    pub constraint: Option<Type>,
    pub default: Option<Type>,
}

#[derive(Debug, Clone)]
pub enum Visibility { Public, Private, Protected }

#[derive(Debug, Clone)]
pub enum ClassMemberKind {
    Field { init: Option<Expr>, type_expr: Option<Type> },
    Method { 
        signatures: Vec<(Vec<(String, Type)>, Type)>,
        body: Option<Vec<Stmt>>, 
        is_async: bool, 
        is_generator: bool, 
    },
    StaticBlock(Vec<Stmt>),
    Getter { body: Vec<Stmt>, return_type: Type },
    Setter { param: (String, Type), body: Vec<Stmt> },
}

#[derive(Debug, Clone)]
pub struct ClassMember {
    pub name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_readonly: bool,
    pub is_abstract: bool,
    pub kind: ClassMemberKind,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, init: Option<Expr> },
    Const { name: String, init: Expr },
    Fn { 
        name: String, 
        params: Vec<(String, Type)>, 
        return_type: Type, 
        body: Vec<Stmt>, 
        is_async: bool, 
        is_generator: bool, 
        decorators: Vec<Decorator> 
    },
    Class { 
        name: String, 
        superclass: Option<String>, 
        members: Vec<ClassMember>, 
        decorators: Vec<Decorator>,
        is_abstract: bool,
    },
    Interface { 
        name: String, 
        type_params: Vec<TypeParam>, 
        extends: Vec<String>,
        body: Vec<Stmt>, 
        index_signature: Option<(String, Type, Type)>,
        callable_signature: Option<(Vec<(String, Type)>, Type)>,
        constructable_signature: Option<(Vec<(String, Type)>, Type)>,
    },
    Enum { name: String, variants: Vec<(String, Option<Type>, Option<Expr>)>, is_const: bool },
    Struct { name: String, fields: Vec<(String, Type)> },
    Trait { name: String, body: Vec<Stmt> },
    Impl { trait_name: Option<String>, type_name: String, body: Vec<Stmt> },
    TypeAlias { name: String, type_params: Vec<TypeParam>, type_expr: Type },
    Macro { name: String, params: Vec<String>, body: String },
    Namespace { name: String, body: Vec<Stmt> },
    Module { name: String, body: Vec<Stmt> },
    Yield { expr: Expr, is_delegate: bool },
    Return(Option<Expr>),
    If { cond: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    For { init: Option<Box<Stmt>>, cond: Option<Expr>, incr: Option<Expr>, body: Vec<Stmt> },
    Try { body: Vec<Stmt>, catch: Option<(String, Vec<Stmt>)>, finally: Option<Vec<Stmt>> },
    Throw(Expr),
    Block(Vec<Stmt>),
    Import { path: String, alias: Option<String> },
    Export(Box<Stmt>),
    Polyglot { lang: String, code: String, input: Option<Expr>, return_var: String },
    Test { name: String, body: Vec<Stmt>, describe: Option<String>, data: Option<Vec<Expr>> },
    Expr(Expr),
    Dialect { name: String, body: Vec<Stmt> },
    UnitDefinition { name: String, base: Option<String>, factor: f64 },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, current: 0 } }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        let mut errors = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => stmts.push(s),
                Err(e) => {
                    errors.push(e);
                    self.synchronize();
                }
            }
        }
        if errors.is_empty() { Ok(stmts) } else { Err(errors.join("\n")) }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon { return; }
            match self.peek().token_type {
                TokenType::Class | TokenType::Fn | TokenType::Let | TokenType::Var | TokenType::Const | 
                TokenType::For | TokenType::If | TokenType::While | TokenType::Return |
                TokenType::Interface | TokenType::Type | TokenType::Enum | TokenType::Macro |
                TokenType::Namespace | TokenType::Module | TokenType::Async | TokenType::Actor |
                TokenType::Protocol | TokenType::Trait | TokenType::Struct | TokenType::Impl |
                TokenType::Test | TokenType::Polyglot | TokenType::Defer | TokenType::Debugger |
                TokenType::Dialect | TokenType::Unit => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::Dialect]) { self.dialect_declaration() }
        else if self.match_token(&[TokenType::Unit]) { self.unit_declaration() }
        else if self.match_token(&[TokenType::Async]) { self.async_declaration() }
        else if self.match_token(&[TokenType::Fn]) { 
            let is_gen = self.match_token(&[TokenType::Star]);
            self.fn_declaration(false, is_gen) 
        }
        else if self.match_token(&[TokenType::Let, TokenType::Var]) { self.let_declaration() }
        else if self.match_token(&[TokenType::Const]) {
            if self.match_token(&[TokenType::Enum]) {
                let mut e = self.enum_statement()?;
                if let Stmt::Enum { is_const, .. } = &mut e {
                    *is_const = true;
                }
                Ok(e)
            } else {
                self.const_declaration()
            }
        }
        else if self.match_token(&[TokenType::Try]) { self.try_statement() }
        else if self.match_token(&[TokenType::For]) { self.for_statement() }
        else if self.match_token(&[TokenType::Import]) { self.import_statement() }
        else if self.match_token(&[TokenType::Export]) { self.export_declaration() }
        else if self.match_token(&[TokenType::Class]) { self.class_declaration(false) }
        else if self.match_token(&[TokenType::Namespace]) { self.namespace_declaration() }
        else if self.match_token(&[TokenType::Module]) { self.module_declaration() }
        else if self.match_token(&[TokenType::Identifier]) && (self.previous().lexeme == "abstract" || self.peek().lexeme == "abstract") {
             if self.peek().lexeme == "abstract" { self.advance(); }
             if self.match_token(&[TokenType::Class]) { self.class_declaration(true) }
             else { self.statement() }
        }
        else if self.match_token(&[TokenType::Interface]) { self.interface_declaration() }
        else if self.match_token(&[TokenType::Type]) { self.type_alias_declaration() }
        else if self.match_token(&[TokenType::Enum]) { self.enum_statement() }
        else if self.match_token(&[TokenType::Struct]) { self.struct_declaration() }
        else if self.match_token(&[TokenType::Trait]) { self.trait_declaration() }
        else if self.match_token(&[TokenType::Test]) { self.test_declaration() }
        else if self.match_token(&[TokenType::Macro]) { self.macro_declaration() }
        else { self.statement() }
    }

    fn dialect_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::String, "Expect dialect name.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let body = self.block()?;
        Ok(Stmt::Dialect { name, body })
    }

    fn unit_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect unit name.")?.lexeme.clone();
        let mut base = None;
        let mut factor = 1.0;
        if self.match_token(&[TokenType::Equal]) {
            let lit = self.advance().literal.clone().ok_or("Expect factor.")?;
            factor = match lit {
                Literal::Number(n) => n,
                _ => return Err("Factor must be a number.".into()),
            };
            base = Some(self.consume(TokenType::Identifier, "Expect base unit.")?.lexeme.clone());
        }
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::UnitDefinition { name, base, factor })
    }

    fn namespace_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        Ok(Stmt::Namespace { name, body: self.block()? })
    }

    fn module_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        Ok(Stmt::Module { name, body: self.block()? })
    }

    fn parse_decorators(&mut self) -> Result<Vec<Decorator>, String> {
        let mut decorators = vec![];
        while self.match_token(&[TokenType::At]) {
            let name = self.consume(TokenType::Identifier, "Expect decorator name.")?.lexeme.clone();
            let mut args = vec![];
            if self.match_token(&[TokenType::LeftParen]) {
                if !self.check(TokenType::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.match_token(&[TokenType::Comma]) { break; }
                    }
                }
                self.consume(TokenType::RightParen, "Expect ')'.")?;
            }
            decorators.push(Decorator { name, args });
        }
        Ok(decorators)
    }

    fn fn_declaration(&mut self, is_async: bool, is_generator: bool) -> Result<Stmt, String> {
        let decorators = self.parse_decorators()?;
        let name = self.consume(TokenType::Identifier, "Expect function name.")?.lexeme.clone();
        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut params = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                let p_name = self.consume(TokenType::Identifier, "Expect parameter name.")?.lexeme.clone();
                self.consume(TokenType::Colon, "Expect ':' after parameter name.")?;
                let p_type = self.parse_type()?;
                params.push((p_name, p_type));
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        let mut return_type = Type::Nil;
        if self.match_token(&[TokenType::Arrow]) {
            return_type = self.parse_type()?;
        }
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let body = self.block()?;
        Ok(Stmt::Fn { name, params, return_type, body, is_async, is_generator, decorators })
    }

    fn class_declaration(&mut self, is_abstract: bool) -> Result<Stmt, String> {
        let decorators = self.parse_decorators()?;
        let name = self.consume(TokenType::Identifier, "Expect class name.")?.lexeme.clone();
        let mut superclass = None;
        if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "extends" {
            superclass = Some(self.consume(TokenType::Identifier, "Expect superclass name.")?.lexeme.clone());
        }
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;
        let mut members = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let mut visibility = Visibility::Public;
            if self.match_token(&[TokenType::Identifier]) {
                visibility = match self.previous().lexeme.as_str() {
                    "private" => Visibility::Private,
                    "protected" => Visibility::Protected,
                    "public" => Visibility::Public,
                    _ => { self.current -= 1; Visibility::Public }
                };
            }
            let is_abstract_member = if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "abstract" { true } else { false };
            let is_static = if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "static" { true } else { false };
            if is_static && self.match_token(&[TokenType::LeftBrace]) {
                members.push(ClassMember { name: "static_block".into(), visibility: Visibility::Public, is_static: true, is_readonly: false, is_abstract: false, kind: ClassMemberKind::StaticBlock(self.block()?) });
                continue;
            }
            let is_readonly = if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "readonly" { true } else { false };
            let mut is_async = false;
            if self.match_token(&[TokenType::Async]) { is_async = true; }
            let m_name = self.consume(TokenType::Identifier, "Expect member name.")?.lexeme.clone();
            if self.match_token(&[TokenType::LeftParen]) {
                let mut params = vec![];
                if !self.check(TokenType::RightParen) {
                    loop {
                        let p_n = self.consume(TokenType::Identifier, "Expect parameter name.")?.lexeme.clone();
                        self.consume(TokenType::Colon, "Expect ':' after parameter name.")?;
                        params.push((p_n, self.parse_type()?));
                        if !self.match_token(&[TokenType::Comma]) { break; }
                    }
                }
                self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
                let mut ret = Type::Nil;
                if self.match_token(&[TokenType::Arrow]) { ret = self.parse_type()?; }
                let body = if self.match_token(&[TokenType::LeftBrace]) { Some(self.block()?) } else { self.consume(TokenType::Semicolon, "Expect ';'.")?; None };
                members.push(ClassMember { name: m_name, visibility, is_static, is_readonly, is_abstract: is_abstract_member, kind: ClassMemberKind::Method { signatures: vec![(params, ret)], body, is_async, is_generator: false } });
            } else {
                let mut t = None;
                if self.match_token(&[TokenType::Colon]) { t = Some(self.parse_type()?); }
                let mut init = None;
                if self.match_token(&[TokenType::Equal]) { init = Some(self.expression()?); }
                self.consume(TokenType::Semicolon, "Expect ';'.")?;
                members.push(ClassMember { name: m_name, visibility, is_static, is_readonly, is_abstract: is_abstract_member, kind: ClassMemberKind::Field { init, type_expr: t } });
            }
        }
        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Ok(Stmt::Class { name, superclass, members, decorators, is_abstract })
    }

    fn interface_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect interface name.")?.lexeme.clone();
        let type_params = self.parse_type_params()?;
        let mut extends = vec![];
        if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "extends" {
            loop {
                extends.push(self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone());
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
        }
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let mut body = vec![];
        while !self.check(TokenType::RightBrace) {
            body.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        Ok(Stmt::Interface { name, type_params, extends, body, index_signature: None, callable_signature: None, constructable_signature: None })
    }

    fn type_alias_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect type alias name.")?.lexeme.clone();
        let type_params = self.parse_type_params()?;
        self.consume(TokenType::Equal, "Expect '='.")?;
        let type_expr = self.parse_type()?;
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::TypeAlias { name, type_params, type_expr })
    }

    fn parse_type_params(&mut self) -> Result<Vec<TypeParam>, String> {
        let mut params = vec![];
        if self.match_token(&[TokenType::Less]) {
            loop {
                let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
                let mut constraint = None;
                if self.match_token(&[TokenType::As]) { constraint = Some(self.parse_type()?); }
                let mut default = None;
                if self.match_token(&[TokenType::Equal]) { default = Some(self.parse_type()?); }
                params.push(TypeParam { name, constraint, default });
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
            self.consume(TokenType::Greater, "Expect '>'.")?;
        }
        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let mut t = self.parse_union_type()?;
        if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "extends" {
            let ext = self.parse_union_type()?;
            self.consume(TokenType::Question, "Expect '?'.")?;
            let tr = self.parse_type()?;
            self.consume(TokenType::Colon, "Expect ':'.")?;
            let fl = self.parse_type()?;
            t = Type::Conditional { check: Box::new(t), extends: Box::new(ext), true_type: Box::new(tr), false_type: Box::new(fl) };
        }
        Ok(t)
    }

    fn parse_union_type(&mut self) -> Result<Type, String> {
        let mut types = vec![self.parse_intersection_type()?];
        while self.match_token(&[TokenType::Pipe]) {
            types.push(self.parse_intersection_type()?);
        }
        if types.len() == 1 { Ok(types.pop().unwrap()) } else { Ok(Type::Union(types)) }
    }

    fn parse_intersection_type(&mut self) -> Result<Type, String> {
        let mut types = vec![self.parse_primary_type()?];
        while self.match_token(&[TokenType::Ampersand]) {
            types.push(self.parse_primary_type()?);
        }
        if types.len() == 1 { Ok(types.pop().unwrap()) } else { Ok(Type::Intersection(types)) }
    }

    fn parse_primary_type(&mut self) -> Result<Type, String> {
        if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "keyof" {
            return Ok(Type::KeyOf(Box::new(self.parse_type()?)));
        }
        if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "infer" {
            return Ok(Type::Infer(self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone()));
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            let mut fields = HashMap::new();
            while !self.check(TokenType::RightBrace) {
                let n = self.consume(TokenType::Identifier, "Expect field.")?.lexeme.clone();
                self.consume(TokenType::Colon, "Expect ':'.")?;
                fields.insert(n, self.parse_type()?);
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
            self.consume(TokenType::RightBrace, "Expect '}'.")?;
            return Ok(Type::Object(fields));
        }
        if self.match_token(&[TokenType::LeftBracket]) {
            let mut types = vec![];
            while !self.check(TokenType::RightBracket) {
                types.push(self.parse_type()?);
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
            self.consume(TokenType::RightBracket, "Expect ']'.")?;
            return Ok(Type::Tuple(types));
        }
        let name_token = self.consume(TokenType::Identifier, "Expect type name.")?.lexeme.clone();
        let mut base = match name_token.as_str() {
            "string" => Type::String, "number" => Type::Number, "bool" => Type::Bool, "nil" => Type::Nil, _ => Type::Named(name_token.clone()),
        };
        if self.match_token(&[TokenType::Less]) {
            let mut args = vec![];
            loop {
                args.push(self.parse_type()?);
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
            self.consume(TokenType::Greater, "Expect '>'.")?;
            base = Type::Generic(name_token, args);
        }
        Ok(base)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::If]) { self.if_statement() }
        else if self.match_token(&[TokenType::While]) { self.while_statement() }
        else if self.match_token(&[TokenType::Return]) { self.return_statement() }
        else if self.match_token(&[TokenType::Yield]) { self.yield_statement() }
        else if self.match_token(&[TokenType::Throw]) { self.throw_statement() }
        else if self.match_token(&[TokenType::Polyglot]) { self.polyglot_statement() }
        else if self.match_token(&[TokenType::LeftBrace]) { Ok(Stmt::Block(self.block()?)) }
        else { self.expression_statement() }
    }

    fn yield_statement(&mut self) -> Result<Stmt, String> {
        let is_delegate = self.match_token(&[TokenType::Star]);
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Yield { expr, is_delegate })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '('.")?;
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')'.")?;
        let then = if self.match_token(&[TokenType::LeftBrace]) { self.block()? } else { vec![self.declaration()?] };
        let mut else_ = None;
        if self.match_token(&[TokenType::Else]) {
            else_ = Some(if self.match_token(&[TokenType::LeftBrace]) { self.block()? } else { vec![self.declaration()?] });
        }
        Ok(Stmt::If { cond, then, else_ })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '('.")?;
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')'.")?;
        let body = if self.match_token(&[TokenType::LeftBrace]) { self.block()? } else { vec![self.declaration()?] };
        Ok(Stmt::While { cond, body })
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let mut value = None;
        if !self.check(TokenType::Semicolon) { value = Some(self.expression()?); }
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Return(value))
    }

    fn throw_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Throw(expr))
    }

    fn polyglot_statement(&mut self) -> Result<Stmt, String> {
        let lang = self.consume(TokenType::Identifier, "Expect language.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let mut code = String::new();
        let mut depth = 1;
        while depth > 0 && !self.is_at_end() {
            if self.check(TokenType::LeftBrace) { depth += 1; }
            if self.check(TokenType::RightBrace) { depth -= 1; }
            if depth > 0 { code.push_str(&self.advance().lexeme); code.push(' '); }
        }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        self.consume(TokenType::As, "Expect 'as'.")?;
        let return_var = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Polyglot { lang, code: code.trim().to_string(), input: None, return_var })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() { stmts.push(self.declaration()?); }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        Ok(stmts)
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> { self.assignment() }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.ternary()?;

        if self.match_token(&[
            TokenType::Equal, TokenType::PlusEqual, TokenType::MinusEqual,
            TokenType::StarEqual, TokenType::SlashEqual, TokenType::ModEqual,
            TokenType::AndEqual, TokenType::OrEqual, TokenType::XorEqual,
            TokenType::ShlEqual, TokenType::ShrEqual, TokenType::PowerEqual,
            TokenType::NullishEqual, TokenType::UShrEqual, TokenType::LogicalAndEqual,
            TokenType::LogicalOrEqual,
        ]) {
            let op = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(n) => {
                    if op.token_type == TokenType::Equal {
                        return Ok(Expr::Assignment { target: Box::new(Expr::Variable(n)), value: Box::new(value) });
                    } else {
                        return Ok(Expr::CompoundAssignment { target: Box::new(Expr::Variable(n)), op, value: Box::new(value) });
                    }
                }
                Expr::Get { object, name } => {
                    if op.token_type == TokenType::Equal {
                        return Ok(Expr::Set { object, name, value: Box::new(value) });
                    } else {
                        return Ok(Expr::CompoundAssignment { target: Box::new(Expr::Get { object, name }), op, value: Box::new(value) });
                    }
                }
                _ => return Err("Invalid assignment target.".into()),
            }
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, String> {
        let mut expr = self.nullish()?;
        if self.match_token(&[TokenType::Question]) {
            let then = self.expression()?;
            self.consume(TokenType::Colon, "Expect ':'.")?;
            let else_ = self.ternary()?;
            expr = Expr::Ternary { cond: Box::new(expr), then: Box::new(then), else_: Box::new(else_) };
        }
        Ok(expr)
    }

    fn nullish(&mut self) -> Result<Expr, String> {
        let mut expr = self.or()?;
        while self.match_token(&[TokenType::Nullish]) {
            let op = self.previous().clone();
            let right = self.or()?;
            expr = Expr::Logical { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.match_token(&[TokenType::LogicalOr]) || (self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "or") {
            let op = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.bitwise_or()?;
        while self.match_token(&[TokenType::LogicalAnd]) || (self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "and") {
            let op = self.previous().clone();
            let right = self.bitwise_or()?;
            expr = Expr::Logical { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn bitwise_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.bitwise_xor()?;
        while self.match_token(&[TokenType::BitOr]) {
            let op = self.previous().clone();
            let right = self.bitwise_xor()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn bitwise_xor(&mut self) -> Result<Expr, String> {
        let mut expr = self.bitwise_and()?;
        while self.match_token(&[TokenType::BitXor]) {
            let op = self.previous().clone();
            let right = self.bitwise_and()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn bitwise_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.match_token(&[TokenType::BitAnd]) {
            let op = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual, TokenType::StrictEqual, TokenType::StrictNotEqual]) {
            let op = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.shift()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual, TokenType::InstanceOf, TokenType::In, TokenType::As, TokenType::Is]) {
            let op = self.previous().clone();
            let right = self.shift()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn shift(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_token(&[TokenType::Shl, TokenType::Shr, TokenType::UShr]) {
            let op = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.power()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star, TokenType::Mod]) {
            let op = self.previous().clone();
            let right = self.power()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Power]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus, TokenType::BitNot, TokenType::TypeOf, TokenType::Increment, TokenType::Decrement]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { op, right: Box::new(right) });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(&[TokenType::LeftParen]) { expr = self.finish_call(expr)?; }
            else if self.match_token(&[TokenType::Dot]) || self.match_token(&[TokenType::OptionalChain]) {
                if self.peek().token_type == TokenType::Identifier {
                    let name = self.consume(TokenType::Identifier, "Expect property.")?.clone();
                    expr = Expr::Get { object: Box::new(expr), name };
                } else if self.peek().token_type == TokenType::CustomOperator {
                    let op = self.advance().clone();
                    let right = self.primary()?;
                    expr = Expr::CustomBinary { left: Box::new(expr), op, right: Box::new(right) };
                } else { break; }
            } else if self.match_token(&[TokenType::CustomOperator]) {
                let op = self.previous().clone();
                let right = self.unary()?;
                expr = Expr::CustomBinary { left: Box::new(expr), op, right: Box::new(right) };
            } else { break; }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut args = vec![];
        if !self.check(TokenType::RightParen) {
            loop { args.push(self.expression()?); if !self.match_token(&[TokenType::Comma]) { break; } }
        }
        self.consume(TokenType::RightParen, "Expect ')'.")?;
        Ok(Expr::Call { callee: Box::new(callee), args })
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::False]) { return Ok(Expr::Literal(Literal::Bool(false))); }
        if self.match_token(&[TokenType::True]) { return Ok(Expr::Literal(Literal::Bool(true))); }
        if self.match_token(&[TokenType::Nil]) { return Ok(Expr::Literal(Literal::Nil)); }
        if self.match_token(&[TokenType::Number, TokenType::String]) { return Ok(Expr::Literal(self.previous().literal.clone().unwrap())); }
        if self.match_token(&[TokenType::This, TokenType::Self_]) { return Ok(Expr::This); }
        if self.match_token(&[TokenType::Super]) {
            self.consume(TokenType::Dot, "Expect '.'.")?;
            let method = self.consume(TokenType::Identifier, "Expect method.")?.clone();
            return Ok(Expr::Super { method });
        }
        if self.match_token(&[
            TokenType::Qoppa, TokenType::Digamma, TokenType::San, TokenType::Koppa,
            TokenType::Sampi, TokenType::Stigma, TokenType::AlphaPsili, TokenType::EpsilonPsili,
            TokenType::OmicronPsili, TokenType::EpsilonDasia, TokenType::IotaPsili,
            TokenType::UpsilonDasia, TokenType::OmegaDasia, TokenType::EtaDasia,
            TokenType::OmicronDasia, TokenType::OmegaPsili
        ]) {
            return self.greek_expression();
        }
        if self.match_token(&[TokenType::Infer]) {
            let name = self.consume(TokenType::Identifier, "Expect name after infer.")?.lexeme.clone();
            return Ok(Expr::Infer(name));
        }
        if self.match_token(&[TokenType::Identifier]) { return Ok(Expr::Variable(self.previous().lexeme.clone())); }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')'.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        if self.match_token(&[TokenType::Match]) { return self.match_expression(); }
        Err(format!("Expect expression at line {}.", self.peek().line))
    }

    fn greek_expression(&mut self) -> Result<Expr, String> {
        let token = self.previous().clone();
        match token.token_type {
            TokenType::Qoppa => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::QuantumSuperposition { expr: Box::new(expr) }))
            }
            TokenType::Digamma => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::QuantumGate { gate: QuantumGate::Hadamard, qubit: Box::new(expr) }))
            }
            TokenType::San => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::FourierTransform { expr: Box::new(expr), inverse: false }))
            }
            TokenType::Koppa => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::Derivative { expr: Box::new(expr), at: None }))
            }
            TokenType::Sampi => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::Integral { expr: Box::new(expr), from: None, to: None }))
            }
            TokenType::Stigma => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::RandomSample { dist: Distribution::Normal }))
            }
            TokenType::AlphaPsili => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::AsyncStart { expr: Box::new(expr) }))
            }
            TokenType::EpsilonPsili => {
                let event = self.expression()?;
                let handler = self.expression()?;
                Ok(Expr::Greek(GreekExpr::EventHandler { event: Box::new(event), handler: Box::new(handler) }))
            }
            TokenType::OmicronPsili => {
                let producer = self.expression()?;
                Ok(Expr::Greek(GreekExpr::Observable { producer: Box::new(producer) }))
            }
            TokenType::IotaPsili => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::Immutable { expr: Box::new(expr) }))
            }
            TokenType::UpsilonDasia => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::Unsafe { expr: Box::new(expr) }))
            }
            TokenType::OmegaDasia => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::WeakRef { expr: Box::new(expr) }))
            }
            TokenType::EtaDasia => {
                let size = self.expression()?;
                Ok(Expr::Greek(GreekExpr::HeapAlloc { size: Box::new(size) }))
            }
            TokenType::OmicronDasia => {
                let expr = self.expression()?;
                Ok(Expr::Greek(GreekExpr::ObjectRef { expr: Box::new(expr) }))
            }
            TokenType::OmegaPsili => {
                let cond = self.expression()?;
                let body = self.block()?;
                Ok(Expr::Greek(GreekExpr::WhileLoop { cond: Box::new(cond), body }))
            }
            _ => Err(format!("Unsupported Greek symbol in expression at line {}.", token.line)),
        }
    }

    fn enum_statement(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let mut variants = vec![];
        loop {
            let v_name = self.consume(TokenType::Identifier, "Expect variant.")?.lexeme.clone();
            let mut v_type = None;
            if self.match_token(&[TokenType::LeftParen]) { v_type = Some(self.parse_type()?); self.consume(TokenType::RightParen, "Expect ')'.")?; }
            let mut v_val = None;
            if self.match_token(&[TokenType::Equal]) { v_val = Some(self.expression()?); }
            variants.push((v_name, v_type, v_val));
            if !self.match_token(&[TokenType::Comma]) { break; }
        }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        Ok(Stmt::Enum { name, variants, is_const: false })
    }

    fn match_expression(&mut self) -> Result<Expr, String> {
        let cond = self.expression()?;
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let mut arms = vec![];
        while !self.check(TokenType::RightBrace) {
            let pattern = self.parse_pattern()?;
            let mut guard = None;
            if self.match_token(&[TokenType::If]) { guard = Some(self.expression()?); }
            self.consume(TokenType::FatArrow, "Expect '=>'.")?;
            arms.push((pattern, guard, self.expression()?));
            self.match_token(&[TokenType::Comma]);
        }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        Ok(Expr::Match { cond: Box::new(cond), arms })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        let mut p = self.parse_single_pattern()?;
        while self.match_token(&[TokenType::Pipe]) {
            let next = self.parse_single_pattern()?;
            if let Pattern::Union(mut variants) = p { variants.push(next); p = Pattern::Union(variants); }
            else { p = Pattern::Union(vec![p, next]); }
        }
        Ok(p)
    }

    fn parse_single_pattern(&mut self) -> Result<Pattern, String> {
        if self.match_token(&[TokenType::Star]) { Ok(Pattern::Wildcard) }
        else if self.match_token(&[TokenType::LeftBracket]) {
            let mut items = vec![];
            while !self.check(TokenType::RightBracket) {
                items.push(self.parse_pattern()?);
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
            self.consume(TokenType::RightBracket, "Expect ']'.")?;
            Ok(Pattern::Array(items, None))
        } else if self.match_token(&[TokenType::Identifier]) {
            Ok(Pattern::Variable(self.previous().lexeme.clone()))
        } else {
            let start = self.advance().literal.clone().ok_or("Expect literal.")?;
            if self.match_token(&[TokenType::Dot]) {
                self.consume(TokenType::Dot, "Expect '..'.")?;
                let inc = self.match_token(&[TokenType::Equal]);
                Ok(Pattern::Range(start, self.advance().literal.clone().ok_or("Expect end.")?, inc))
            } else { Ok(Pattern::Literal(start)) }
        }
    }

    fn async_declaration(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Fn, "Expect 'fn'.")?;
        let is_gen = self.match_token(&[TokenType::Star]);
        self.fn_declaration(true, is_gen)
    }

    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        let mut init = None;
        if self.match_token(&[TokenType::Equal]) { init = Some(self.expression()?); }
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Let { name, init })
    }

    fn const_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::Equal, "Expect '='.")?;
        let init = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Const { name, init })
    }

    fn try_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let body = self.block()?;
        let mut catch = None;
        if self.match_token(&[TokenType::Catch]) {
            self.consume(TokenType::LeftParen, "Expect '('.")?;
            let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
            self.consume(TokenType::RightParen, "Expect ')'.")?;
            self.consume(TokenType::LeftBrace, "Expect '{'.")?;
            catch = Some((name, self.block()?));
        }
        let mut finally = None;
        if self.match_token(&[TokenType::Finally]) {
            self.consume(TokenType::LeftBrace, "Expect '{'.")?;
            finally = Some(self.block()?);
        }
        Ok(Stmt::Try { body, catch, finally })
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '('.")?;
        let init = if self.match_token(&[TokenType::Semicolon]) { None }
                   else if self.match_token(&[TokenType::Let]) { Some(Box::new(self.let_declaration()?)) }
                   else { Some(Box::new(self.expression_statement()?)) };
        let cond = if !self.check(TokenType::Semicolon) { Some(self.expression()?) } else { None };
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        let incr = if !self.check(TokenType::RightParen) { Some(self.expression()?) } else { None };
        self.consume(TokenType::RightParen, "Expect ')'.")?;
        let body = if self.match_token(&[TokenType::LeftBrace]) { self.block()? } else { vec![self.declaration()?] };
        Ok(Stmt::For { init, cond, incr, body })
    }

    fn import_statement(&mut self) -> Result<Stmt, String> {
        let path = self.consume(TokenType::String, "Expect path.")?.lexeme.clone();
        let mut alias = None;
        if self.match_token(&[TokenType::As]) { alias = Some(self.consume(TokenType::Identifier, "Expect alias.")?.lexeme.clone()); }
        self.consume(TokenType::Semicolon, "Expect ';'.")?;
        Ok(Stmt::Import { path, alias })
    }

    fn test_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::String, "Expect name.")?.lexeme.clone();
        let mut describe = None;
        if self.match_token(&[TokenType::Identifier]) && self.previous().lexeme == "describe" {
            describe = Some(self.consume(TokenType::String, "Expect name.")?.lexeme.clone());
        }
        let mut data = None;
        if self.match_token(&[TokenType::LeftBracket]) {
            let mut vals = vec![];
            loop { vals.push(self.expression()?); if !self.match_token(&[TokenType::Comma]) { break; } }
            self.consume(TokenType::RightBracket, "Expect ']'.")?;
            data = Some(vals);
        }
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        Ok(Stmt::Test { name, body: self.block()?, describe, data })
    }

    fn macro_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::LeftParen, "Expect '('.")?;
        let mut params = vec![];
        if !self.check(TokenType::RightParen) {
            loop { params.push(self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone()); if !self.match_token(&[TokenType::Comma]) { break; } }
        }
        self.consume(TokenType::RightParen, "Expect ')'.")?;
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let mut body = String::new();
        let mut depth = 1;
        while depth > 0 && !self.is_at_end() {
            if self.check(TokenType::LeftBrace) { depth += 1; }
            if self.check(TokenType::RightBrace) { depth -= 1; }
            if depth > 0 { body.push_str(&self.advance().lexeme); body.push(' '); }
        }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        Ok(Stmt::Macro { name, params, body: body.trim().to_string() })
    }

    fn trait_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let body = self.block()?;
        Ok(Stmt::Trait { name, body })
    }

    fn struct_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect name.")?.lexeme.clone();
        self.consume(TokenType::LeftBrace, "Expect '{'.")?;
        let mut fields = vec![];
        while !self.check(TokenType::RightBrace) {
            let n = self.consume(TokenType::Identifier, "Expect field.")?.lexeme.clone();
            self.consume(TokenType::Colon, "Expect ':'.")?;
            fields.push((n, self.parse_type()?));
            if !self.match_token(&[TokenType::Comma]) { break; }
        }
        self.consume(TokenType::RightBrace, "Expect '}'.")?;
        Ok(Stmt::Struct { name, fields })
    }

    fn export_declaration(&mut self) -> Result<Stmt, String> { Ok(Stmt::Export(Box::new(self.declaration()?))) }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types { if self.check(t.clone()) { self.advance(); return true; } }
        false
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(t) { Ok(self.advance()) } else { Err(format!("{} at line {}.", msg, self.peek().line)) }
    }

    fn check(&self, t: TokenType) -> bool { if self.is_at_end() { false } else { self.peek().token_type == t } }

    fn advance(&mut self) -> &Token { if !self.is_at_end() { self.current += 1; } self.previous() }

    fn is_at_end(&self) -> bool { self.peek().token_type == TokenType::Eof }

    fn peek(&self) -> &Token { &self.tokens[self.current] }

    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }
}
