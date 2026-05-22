// navescript/src/types.rs
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Bool,
    Nil,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Object(HashMap<String, Type>),
    Function { params: Vec<Type>, return_type: Box<Type> },
    Union(Vec<Type>),
    Intersection(Vec<Type>),
    Generic(String, Vec<Type>),        // Base<T, U>
    TypeVar(String),                   // 't0 (inference var)
    TypeParameter(String),             // T (declared generic param)
    Class(String),
    Named(String),
    Conditional {
        check: Box<Type>,
        extends: Box<Type>,
        true_type: Box<Type>,
        false_type: Box<Type>,
    },
    Infer(String),                     // `infer U`
    KeyOf(Box<Type>),
    Mapped {
        param: String,
        source: Box<Type>,
        target: Box<Type>,
    },
    Dimensional(Box<Type>, String),    // e.g., 5.0 "m" (unit-aware)
    Reactive(Box<Type>),
    Table(HashMap<String, Type>),
    Immutable(Box<Type>),
    Unsafe(Box<Type>),
    Weak(Box<Type>),
    HeapAlloc(Box<Type>),
    Literal(crate::ns_lexer::Literal),
    Unknown,
}

impl std::hash::Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Type::Number | Type::String | Type::Bool | Type::Nil | Type::Unknown => {},
            Type::Array(inner) | Type::KeyOf(inner) | Type::Reactive(inner) | Type::Immutable(inner) | Type::Unsafe(inner) | Type::Weak(inner) | Type::HeapAlloc(inner) => inner.hash(state),
            Type::Dimensional(inner, unit) => { inner.hash(state); unit.hash(state); }
            Type::Object(fields) | Type::Table(fields) => {
                let mut sorted_keys: Vec<_> = fields.keys().collect();
                sorted_keys.sort();
                for k in sorted_keys {
                    k.hash(state);
                    fields.get(k).unwrap().hash(state);
                }
            }
            Type::Function { params, return_type } => {
                params.hash(state);
                return_type.hash(state);
            }
            Type::Union(variants) | Type::Intersection(variants) | Type::Tuple(variants) => variants.hash(state),
            Type::Generic(base, args) => {
                base.hash(state);
                args.hash(state);
            }
            Type::TypeParameter(s) | Type::TypeVar(s) | Type::Class(s) | Type::Named(s) | Type::Infer(s) => s.hash(state),
            Type::Mapped { param, source, target } => {
                param.hash(state);
                source.hash(state);
                target.hash(state);
            }
            Type::Conditional { check, extends, true_type, false_type } => {
                check.hash(state);
                extends.hash(state);
                true_type.hash(state);
                false_type.hash(state);
            }
            Type::Literal(l) => l.hash(state),
        }
    }
}

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub type Substitution = HashMap<String, Type>;

#[derive(Default)]
pub struct TypeChecker {
    pub substitution: Substitution,
    pub next_var_id: usize,
    pub symbol_table: HashMap<String, Type>,
    pub generic_context: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self { Self::default() }

    fn fresh_var(&mut self) -> Type {
        let name = format!("'t{}", self.next_var_id);
        self.next_var_id += 1;
        Type::TypeVar(name)
    }

    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), TypeError> {
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        match (&t1, &t2) {
            (Type::TypeVar(v), t) | (t, Type::TypeVar(v)) => {
                if self.occurs_check(v, t) {
                    return Err(TypeError { message: format!("Infinite type: {}", v), line: 0, column: 0 });
                }
                self.substitution.insert(v.clone(), t.clone());
                Ok(())
            }
            (Type::Number, Type::Number) |
            (Type::String, Type::String) |
            (Type::Bool, Type::Bool) |
            (Type::Nil, Type::Nil) => Ok(()),

            (Type::Array(a1), Type::Array(a2)) => self.unify(a1, a2),
            (Type::Tuple(t1), Type::Tuple(t2)) if t1.len() == t2.len() => {
                for (a, b) in t1.iter().zip(t2.iter()) { self.unify(a, b)?; }
                Ok(())
            }
            (Type::Function { params: p1, return_type: r1 },
             Type::Function { params: p2, return_type: r2 }) => {
                if p1.len() != p2.len() { return Err(TypeError { message: "Arity mismatch".into(), line: 0, column: 0 }); }
                for (a, b) in p1.iter().zip(p2.iter()) { self.unify(a, b)?; }
                self.unify(r1, r2)
            }
            (Type::Dimensional(base1, unit1), Type::Dimensional(base2, unit2)) if unit1 == unit2 => {
                self.unify(base1, base2)
            }
            (Type::Reactive(i1), Type::Reactive(i2)) => self.unify(i1, i2),
            (Type::Immutable(i1), Type::Immutable(i2)) => self.unify(i1, i2),
            (Type::Unsafe(i1), Type::Unsafe(i2)) => self.unify(i1, i2),
            (Type::Weak(i1), Type::Weak(i2)) => self.unify(i1, i2),
            (Type::HeapAlloc(i1), Type::HeapAlloc(i2)) => self.unify(i1, i2),
            _ => Ok(()), // Simplified for brevity
        }
    }

    fn occurs_check(&self, var: &str, t: &Type) -> bool {
        match t {
            Type::TypeVar(v) => v == var,
            Type::Array(inner) | Type::KeyOf(inner) | Type::Reactive(inner) | Type::Immutable(inner) | Type::Unsafe(inner) | Type::Weak(inner) | Type::HeapAlloc(inner) | Type::Dimensional(inner, _) => self.occurs_check(var, inner),
            Type::Infer(s) => s == var,
            Type::Function { params, return_type } => params.iter().any(|p| self.occurs_check(var, p)) || self.occurs_check(var, return_type),
            Type::Tuple(items) | Type::Union(items) | Type::Intersection(items) => items.iter().any(|i| self.occurs_check(var, i)),
            _ => false,
        }
    }

    fn apply(&self, t: &Type) -> Type {
        match t {
            Type::TypeVar(v) => self.substitution.get(v).cloned().unwrap_or_else(|| t.clone()),
            Type::Array(inner) => Type::Array(Box::new(self.apply(inner))),
            Type::Tuple(items) => Type::Tuple(items.iter().map(|i| self.apply(i)).collect()),
            Type::Dimensional(inner, u) => Type::Dimensional(Box::new(self.apply(inner)), u.clone()),
            Type::Reactive(inner) => Type::Reactive(Box::new(self.apply(inner))),
            Type::Immutable(inner) => Type::Immutable(Box::new(self.apply(inner))),
            Type::Unsafe(inner) => Type::Unsafe(Box::new(self.apply(inner))),
            Type::Weak(inner) => Type::Weak(Box::new(self.apply(inner))),
            Type::HeapAlloc(inner) => Type::HeapAlloc(Box::new(self.apply(inner))),
            Type::Function { params, return_type } => Type::Function {
                params: params.iter().map(|p| self.apply(p)).collect(),
                return_type: Box::new(self.apply(return_type)),
            },
            _ => t.clone(),
        }
    }

    pub fn infer_expr(&mut self, expr: &crate::ns_parser::Expr) -> Result<Type, TypeError> {
        match expr {
            crate::ns_parser::Expr::Literal(lit) => match lit {
                crate::ns_lexer::Literal::Number(_) => Ok(Type::Number),
                crate::ns_lexer::Literal::String(_) => Ok(Type::String),
                crate::ns_lexer::Literal::Bool(_) => Ok(Type::Bool),
                crate::ns_lexer::Literal::Nil => Ok(Type::Nil),
                crate::ns_lexer::Literal::UnitValue(_, unit) => Ok(Type::Dimensional(Box::new(Type::Number), unit.clone())),
                _ => Ok(Type::Unknown),
            },
            crate::ns_parser::Expr::Variable(name) => {
                self.symbol_table.get(name).cloned()
                    .ok_or_else(|| TypeError { message: format!("Undefined variable: {}", name), line: 0, column: 0 })
            }
            crate::ns_parser::Expr::Binary { left, op, right } => {
                let lt = self.infer_expr(left)?;
                let rt = self.infer_expr(right)?;
                match op.token_type {
                    crate::ns_lexer::TokenType::Plus | crate::ns_lexer::TokenType::Minus => {
                        self.unify(&lt, &rt)?;
                        Ok(lt)
                    }
                    _ => Ok(self.fresh_var()),
                }
            }
            crate::ns_parser::Expr::Call { callee, args } => {
                let ct = self.infer_expr(callee)?;
                if let Type::Function { params, return_type } = ct {
                    if params.len() != args.len() {
                        return Err(TypeError { message: "Arity mismatch".into(), line: 0, column: 0 });
                    }
                    for (p, a) in params.iter().zip(args.iter()) {
                        let at = self.infer_expr(a)?;
                        self.unify(p, &at)?;
                    }
                    Ok(*return_type)
                } else {
                    Err(TypeError { message: "Not callable".into(), line: 0, column: 0 })
                }
            }
            _ => Ok(self.fresh_var()),
        }
    }

    pub fn check_stmt(&mut self, stmt: &crate::ns_parser::Stmt) -> Result<(), TypeError> {
        match stmt {
            crate::ns_parser::Stmt::Let { name, init: Some(expr) } => {
                let t = self.infer_expr(expr)?;
                self.symbol_table.insert(name.clone(), t);
            }
            crate::ns_parser::Stmt::Fn { name, params, return_type, .. } => {
                let fn_type = Type::Function {
                    params: params.iter().map(|(_, t)| t.clone()).collect(),
                    return_type: Box::new(return_type.clone()),
                };
                self.symbol_table.insert(name.clone(), fn_type);
            }
            _ => {}
        }
        Ok(())
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n { dp[i][0] = i; }
    for j in 0..=m { dp[0][j] = j; }
    for i in 1..=n {
        for j in 1..=m {
            let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(dp[i-1][j] + 1, std::cmp::min(dp[i][j-1] + 1, dp[i-1][j-1] + cost));
        }
    }
    dp[n][m]
}
