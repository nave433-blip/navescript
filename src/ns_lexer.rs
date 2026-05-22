// navescript/src/ns_lexer.rs
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket, Comma, Dot, Semicolon, Colon, Arrow, FatArrow, Pipe, Ampersand, At, Question,
    DotDot, DotDotEqual, Spread, ColonEqual, Hash, Dollar, Backslash, Backtick,
    // Operators
    Minus, Plus, Slash, Star, Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,
    StrictEqual, StrictNotEqual, Increment, Decrement,
    BitAnd, BitOr, BitXor, BitNot, Shl, Shr, UShr, Mod, Power,
    PlusEqual, MinusEqual, StarEqual, SlashEqual, ModEqual, AndEqual, OrEqual, XorEqual, ShlEqual, ShrEqual, PowerEqual, NullishEqual, UShrEqual, LogicalAndEqual, LogicalOrEqual,
    LogicalAnd, LogicalOr, Nullish, OptionalChain, Pipeline, Bind,
    // Keywords
    Let, Const, Var, Fn, If, Else, While, For, Return, Break, Continue, Match, Try, Catch, Finally, Async, Await, Import, Export, Class, Interface, Type, Enum, Struct, Impl, Trait, Polyglot, As, Test, Throw, Macro, Yield, New,
    Namespace, Module, Super, This, Self_, InstanceOf, TypeOf, Is, In, Of, With, From, Default,
    Debugger, Delete, Do, Static, Readonly, Abstract, Public, Private, Protected, Virtual, Override, Operator,
    Extension, Protocol, Actor, Defer, Fallthrough, Where, Dynamic, External, TypeParam,
    Never, Unknown, Any, Void,
    Dialect, CustomOperator, Unit, Infer,
    Fuzzer, Rule, Breakpoint, Watch, Step, Taint, Source, Sink, Flow, Audit,
    // Greek Symbols (Quantum, Signal, Math)
    Qoppa, Digamma, San, Koppa, Sampi, Kai, IotaDiaeresis, UpsilonDiaeresis,
    BetaCursive, ThetaCursive, PhiCursive, PiCursive, RhoCursive, Stigma,
    AlphaPsili, EpsilonPsili, OmicronPsili, EpsilonDasia, IotaPsili, UpsilonDasia, OmegaDasia, EtaDasia, OmicronDasia, OmegaPsili,
    // Literals
    Identifier, String, Number, True, False, Nil, Regex, TemplateString,
    Eof,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Literal {
    Number(f64),
    Rational(i64, i64), // numerator, denominator
    Complex(f64, f64),  // real, imaginary
    String(String),
    Bool(bool),
    Nil,
    Regex(String),
    TemplateString(String),
    UnitValue(f64, String), // value, unit (e.g., 5.0, "m")
}

impl Eq for Literal {}

impl std::hash::Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Literal::Number(n) => n.to_bits().hash(state),
            Literal::Rational(n, d) => { n.hash(state); d.hash(state); }
            Literal::Complex(r, i) => { r.to_bits().hash(state); i.to_bits().hash(state); }
            Literal::String(s) => s.hash(state),
            Literal::Bool(b) => b.hash(state),
            Literal::Nil => 0.hash(state),
            Literal::Regex(r) => r.hash(state),
            Literal::TemplateString(t) => t.hash(state),
            Literal::UnitValue(v, u) => { v.to_bits().hash(state); u.hash(state); }
        }
    }
}

impl Literal {
    pub fn as_number(&self) -> Option<f64> {
        if let Literal::Number(n) = self { Some(*n) } else { None }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("const".to_string(), TokenType::Const);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("fn".to_string(), TokenType::Fn);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("match".to_string(), TokenType::Match);
        keywords.insert("try".to_string(), TokenType::Try);
        keywords.insert("catch".to_string(), TokenType::Catch);
        keywords.insert("finally".to_string(), TokenType::Finally);
        keywords.insert("async".to_string(), TokenType::Async);
        keywords.insert("await".to_string(), TokenType::Await);
        keywords.insert("import".to_string(), TokenType::Import);
        keywords.insert("export".to_string(), TokenType::Export);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("interface".to_string(), TokenType::Interface);
        keywords.insert("type".to_string(), TokenType::Type);
        keywords.insert("enum".to_string(), TokenType::Enum);
        keywords.insert("struct".to_string(), TokenType::Struct);
        keywords.insert("impl".to_string(), TokenType::Impl);
        keywords.insert("trait".to_string(), TokenType::Trait);
        keywords.insert("polyglot".to_string(), TokenType::Polyglot);
        keywords.insert("as".to_string(), TokenType::As);
        keywords.insert("test".to_string(), TokenType::Test);
        keywords.insert("throw".to_string(), TokenType::Throw);
        keywords.insert("macro".to_string(), TokenType::Macro);
        keywords.insert("yield".to_string(), TokenType::Yield);
        keywords.insert("new".to_string(), TokenType::New);
        keywords.insert("namespace".to_string(), TokenType::Namespace);
        keywords.insert("module".to_string(), TokenType::Module);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("self".to_string(), TokenType::Self_);
        keywords.insert("instanceof".to_string(), TokenType::InstanceOf);
        keywords.insert("typeof".to_string(), TokenType::TypeOf);
        keywords.insert("is".to_string(), TokenType::Is);
        keywords.insert("in".to_string(), TokenType::In);
        keywords.insert("of".to_string(), TokenType::Of);
        keywords.insert("with".to_string(), TokenType::With);
        keywords.insert("from".to_string(), TokenType::From);
        keywords.insert("default".to_string(), TokenType::Default);
        keywords.insert("debugger".to_string(), TokenType::Debugger);
        keywords.insert("delete".to_string(), TokenType::Delete);
        keywords.insert("do".to_string(), TokenType::Do);
        keywords.insert("static".to_string(), TokenType::Static);
        keywords.insert("readonly".to_string(), TokenType::Readonly);
        keywords.insert("abstract".to_string(), TokenType::Abstract);
        keywords.insert("public".to_string(), TokenType::Public);
        keywords.insert("private".to_string(), TokenType::Private);
        keywords.insert("protected".to_string(), TokenType::Protected);
        keywords.insert("virtual".to_string(), TokenType::Virtual);
        keywords.insert("override".to_string(), TokenType::Override);
        keywords.insert("operator".to_string(), TokenType::Operator);
        keywords.insert("extension".to_string(), TokenType::Extension);
        keywords.insert("protocol".to_string(), TokenType::Protocol);
        keywords.insert("actor".to_string(), TokenType::Actor);
        keywords.insert("defer".to_string(), TokenType::Defer);
        keywords.insert("fallthrough".to_string(), TokenType::Fallthrough);
        keywords.insert("where".to_string(), TokenType::Where);
        keywords.insert("dynamic".to_string(), TokenType::Dynamic);
        keywords.insert("external".to_string(), TokenType::External);
        keywords.insert("never".to_string(), TokenType::Never);
        keywords.insert("unknown".to_string(), TokenType::Unknown);
        keywords.insert("any".to_string(), TokenType::Any);
        keywords.insert("void".to_string(), TokenType::Void);
        keywords.insert("dialect".to_string(), TokenType::Dialect);
        keywords.insert("unit".to_string(), TokenType::Unit);
        keywords.insert("infer".to_string(), TokenType::Infer);
        keywords.insert("fuzzer".to_string(), TokenType::Fuzzer);
        keywords.insert("rule".to_string(), TokenType::Rule);
        keywords.insert("breakpoint".to_string(), TokenType::Breakpoint);
        keywords.insert("watch".to_string(), TokenType::Watch);
        keywords.insert("step".to_string(), TokenType::Step);
        keywords.insert("taint".to_string(), TokenType::Taint);
        keywords.insert("source".to_string(), TokenType::Source);
        keywords.insert("sink".to_string(), TokenType::Sink);
        keywords.insert("flow".to_string(), TokenType::Flow);
        keywords.insert("audit".to_string(), TokenType::Audit);

        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            ';' => self.add_token(TokenType::Semicolon),
            // Greek Symbols
            'Ϙ' => self.add_token(TokenType::Qoppa),
            'Ϝ' => self.add_token(TokenType::Digamma),
            'Ϻ' => self.add_token(TokenType::San),
            'Ϟ' => self.add_token(TokenType::Koppa),
            'Ϡ' => self.add_token(TokenType::Sampi),
            'Ϗ' => self.add_token(TokenType::Kai),
            'Ϊ' => self.add_token(TokenType::IotaDiaeresis),
            'Ϋ' => self.add_token(TokenType::UpsilonDiaeresis),
            'ϐ' => self.add_token(TokenType::BetaCursive),
            'ϑ' => self.add_token(TokenType::ThetaCursive),
            'ϕ' => self.add_token(TokenType::PhiCursive),
            'ϖ' => self.add_token(TokenType::PiCursive),
            'ϱ' => self.add_token(TokenType::RhoCursive),
            'ϛ' => self.add_token(TokenType::Stigma),
            'ἀ' => self.add_token(TokenType::AlphaPsili),
            'ἑ' => self.add_token(TokenType::EpsilonPsili),
            'ὀ' => self.add_token(TokenType::OmicronPsili),
            'ἕ' => self.add_token(TokenType::EpsilonDasia),
            'ἰ' => self.add_token(TokenType::IotaPsili),
            'ὑ' => self.add_token(TokenType::UpsilonDasia),
            'ὡ' => self.add_token(TokenType::OmegaDasia),
            'ἡ' => self.add_token(TokenType::EtaDasia),
            'ὁ' => self.add_token(TokenType::OmicronDasia),
            'ὠ' => self.add_token(TokenType::OmegaPsili),

            // Flexible Multi-char Operators
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            c if is_operator_char(c) => self.operator(),
            _ => return Err(format!("Unexpected character '{}' at line {}.", c, self.line)),
        }
        Ok(())
    }

    fn operator(&mut self) {
        while is_operator_char(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = match text.as_str() {
            "." => TokenType::Dot,
            ".." => TokenType::DotDot,
            "..." => TokenType::Spread,
            "+" => TokenType::Plus,
            "+=" => TokenType::PlusEqual,
            "++" => TokenType::Increment,
            "-" => TokenType::Minus,
            "-=" => TokenType::MinusEqual,
            "--" => TokenType::Decrement,
            "*" => TokenType::Star,
            "*=" => TokenType::StarEqual,
            "**" => TokenType::Power,
            "**=" => TokenType::PowerEqual,
            "/" => TokenType::Slash,
            "/=" => TokenType::SlashEqual,
            "%" => TokenType::Mod,
            "%=" => TokenType::ModEqual,
            "=" => TokenType::Equal,
            "==" => TokenType::EqualEqual,
            "===" => TokenType::StrictEqual,
            "=>" => TokenType::FatArrow,
            "!" => TokenType::Bang,
            "!=" => TokenType::BangEqual,
            "!==" => TokenType::StrictNotEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            "<<" => TokenType::Shl,
            "<<=" => TokenType::ShlEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            ">>" => TokenType::Shr,
            ">>=" => TokenType::ShrEqual,
            ">>>" => TokenType::UShr,
            ">>>=" => TokenType::UShrEqual,
            "&" => TokenType::BitAnd,
            "&=" => TokenType::AndEqual,
            "&&" => TokenType::LogicalAnd,
            "&&=" => TokenType::LogicalAndEqual,
            "|" => TokenType::BitOr,
            "|=" => TokenType::OrEqual,
            "||" => TokenType::LogicalOr,
            "||=" => TokenType::LogicalOrEqual,
            "|>" => TokenType::Pipeline,
            "^" => TokenType::BitXor,
            "^=" => TokenType::XorEqual,
            "~" => TokenType::BitNot,
            "?" => TokenType::Question,
            "??" => TokenType::Nullish,
            "??=" => TokenType::NullishEqual,
            "?." => TokenType::OptionalChain,
            ":" => TokenType::Colon,
            ":=" => TokenType::ColonEqual,
            "::" => TokenType::Bind,
            "#" => TokenType::Hash,
            "$" => TokenType::Dollar,
            "\\" => TokenType::Backslash,
            "`" => TokenType::Backtick,
            "@" => TokenType::At,
            _ => TokenType::CustomOperator,
        };
        self.add_token(token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
            line: self.line,
        });
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string.".to_string());
        }

        self.advance(); // The closing "

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_with_literal(TokenType::String, Some(Literal::String(value)));
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value_str: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = value_str.parse().unwrap();

        // Check for units (e.g. 5m, 10s, 100px)
        if self.peek().is_alphabetic() {
            let unit_start = self.current;
            while self.peek().is_alphabetic() {
                self.advance();
            }
            let unit: String = self.source[unit_start..self.current].iter().collect();
            self.add_token_with_literal(TokenType::Number, Some(Literal::UnitValue(value, unit)));
            return;
        }

        if self.match_char('r') {
            let mut den_str = String::new();
            while self.peek().is_ascii_digit() {
                den_str.push(self.advance());
            }
            if !den_str.is_empty() {
                self.add_token_with_literal(TokenType::Number, Some(Literal::Rational(value as i64, den_str.parse().unwrap())));
                return;
            }
        }

        if self.match_char('i') {
            self.add_token_with_literal(TokenType::Number, Some(Literal::Complex(0.0, value)));
            return;
        }

        self.add_token_with_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = self.keywords.get(&text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }
}

fn is_operator_char(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' | '?' | ':' | '@' | '#' | '$' | '\\' | '`' | '.' => true,
        _ => false,
    }
}
