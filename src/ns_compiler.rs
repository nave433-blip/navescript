// navescript/src/ns_compiler.rs
use crate::ns_parser::{Stmt, Expr};
use crate::ir::{StepIr, Instruction};

pub struct TargetCompiler {
    pub ir: Vec<StepIr>,
    next_id: u32,
}

impl TargetCompiler {
    pub fn new() -> Self {
        Self { ir: Vec::new(), next_id: 1 }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) -> Vec<StepIr> {
        let mut body = Vec::new();
        for stmt in stmts {
            self.compile_stmt(stmt, &mut body);
        }
        body
    }

    pub fn ir_to_wat(ir: &[StepIr]) -> String {
        let mut wat = String::from("(module\n  (import \"env\" \"nave_print\" (func $nave_print (param i32 i32)))\n  (memory (export \"memory\") 1)\n  (func (export \"main\")\n");
        
        for step in ir {
            match &step.instr {
                Instruction::Log(msg) => {
                    wat.push_str(&format!("    ;; Log: {}\n", msg));
                }
                Instruction::NativeOp { name, .. } => {
                    wat.push_str(&format!("    ;; NativeOp: {}\n", name));
                }
                _ => {
                    wat.push_str("    ;; Instruction placeholder\n");
                }
            }
        }
        
        wat.push_str("  )\n)\n");
        wat
    }

    fn add_step(&mut self, instr: Instruction, body: &mut Vec<StepIr>) {
        body.push(StepIr {
            id: Some(self.next_id.to_string()),
            instr,
            next: Some((self.next_id + 1).to_string()),
            on_error: None,
        });
        self.next_id += 1;
    }

    fn compile_stmt(&mut self, stmt: &Stmt, body: &mut Vec<StepIr>) {
        match stmt {
            Stmt::Let { name, init } => {
                if let Some(expr) = init {
                    self.compile_expr(expr, body);
                    self.add_step(Instruction::NativeOp { 
                        name: "set_var".into(), 
                        args: vec![name.clone(), "stack".into()],
                        return_var: "void".into()
                    }, body);
                }
            }
            Stmt::Const { name, init } => {
                self.compile_expr(init, body);
                self.add_step(Instruction::NativeOp { 
                    name: "set_var".into(), 
                    args: vec![name.clone(), "stack".into()],
                    return_var: "void".into()
                }, body);
            }
            Stmt::Fn { name, .. } => {
                self.add_step(Instruction::Log(format!("fn {}", name)), body);
            }
            Stmt::Class { name, .. } => {
                self.add_step(Instruction::Log(format!("class {}", name)), body);
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_expr(e, body);
                    self.add_step(Instruction::Return { value_var: Some("stack".into()) }, body);
                } else {
                    self.add_step(Instruction::Return { value_var: None }, body);
                }
            }
            Stmt::If { cond, then, else_ } => {
                self.compile_expr(cond, body);
                for s in then { self.compile_stmt(s, body); }
                if let Some(els) = else_ {
                    for s in els { self.compile_stmt(s, body); }
                }
            }
            Stmt::While { cond, body: while_body } => {
                self.compile_expr(cond, body);
                for s in while_body { self.compile_stmt(s, body); }
            }
            Stmt::For { init, cond, incr, body: for_body } => {
                if let Some(i) = init { self.compile_stmt(i, body); }
                if let Some(c) = cond { self.compile_expr(c, body); }
                for s in for_body { self.compile_stmt(s, body); }
                if let Some(ic) = incr { self.compile_expr(ic, body); }
            }
            Stmt::Block(stmts) => {
                for s in stmts { self.compile_stmt(s, body); }
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr, body);
            }
            Stmt::Namespace { name, body: ns_body } => {
                self.add_step(Instruction::Log(format!("namespace {}", name)), body);
                for s in ns_body { self.compile_stmt(s, body); }
            }
            Stmt::Module { name, body: mod_body } => {
                self.add_step(Instruction::Log(format!("module {}", name)), body);
                for s in mod_body { self.compile_stmt(s, body); }
            }
            Stmt::Yield { expr, .. } => {
                self.compile_expr(expr, body);
                self.add_step(Instruction::NativeOp {
                    name: "yield".into(),
                    args: vec!["stack".into()],
                    return_var: "stack".into()
                }, body);
            }
            Stmt::Throw(expr) => {
                self.compile_expr(expr, body);
                self.add_step(Instruction::Throw { error_var: "stack".into() }, body);
            }
            Stmt::Try { body: try_body, catch, finally } => {
                for s in try_body { self.compile_stmt(s, body); }
                if let Some((_, catch_body)) = catch {
                    for s in catch_body { self.compile_stmt(s, body); }
                }
                if let Some(finally_body) = finally {
                    for s in finally_body { self.compile_stmt(s, body); }
                }
            }
            Stmt::Import { .. } | Stmt::Export(_) | Stmt::Interface { .. } | Stmt::TypeAlias { .. } | Stmt::Enum { .. } | Stmt::Struct { .. } | Stmt::Trait { .. } | Stmt::Impl { .. } | Stmt::Macro { .. } | Stmt::Test { .. } => {}
            Stmt::Polyglot { lang, code, .. } => {
                self.add_step(Instruction::PolyglotEval {
                    lang: lang.clone(),
                    code: code.clone(),
                    input_var: None,
                    return_var: "stack".into(),
                }, body);
            }
            Stmt::Dialect { name, body: dialect_body } => {
                self.add_step(Instruction::Log(format!("switching to dialect {}", name)), body);
                for s in dialect_body { self.compile_stmt(s, body); }
            }
            Stmt::UnitDefinition { name, factor, .. } => {
                self.add_step(Instruction::Log(format!("defining unit {} with factor {}", name, factor)), body);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, body: &mut Vec<StepIr>) {
        match expr {
            Expr::Literal(lit) => {
                self.add_step(Instruction::SetVar { 
                    var: "stack".into(),
                    value: serde_json::to_value(lit).unwrap() 
                }, body);
            }
            Expr::Variable(name) => {
                self.add_step(Instruction::NativeOp {
                    name: "get_var".into(),
                    args: vec![name.clone()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::This => {
                self.add_step(Instruction::NativeOp {
                    name: "get_this".into(),
                    args: vec![],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Super { method } => {
                self.add_step(Instruction::NativeOp {
                    name: "get_super".into(),
                    args: vec![method.lexeme.clone()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(left, body);
                self.compile_expr(right, body);
                self.add_step(Instruction::NativeOp { 
                    name: "binary_op".into(), 
                    args: vec![op.lexeme.clone(), "stack_1".into(), "stack_0".into()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Logical { left, op, right } => {
                self.compile_expr(left, body);
                self.compile_expr(right, body);
                self.add_step(Instruction::NativeOp { 
                    name: "logical_op".into(), 
                    args: vec![op.lexeme.clone(), "stack_1".into(), "stack_0".into()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Unary { op, right } => {
                self.compile_expr(right, body);
                self.add_step(Instruction::NativeOp { 
                    name: "unary_op".into(), 
                    args: vec![op.lexeme.clone(), "stack".into()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Assignment { target, value } => {
                self.compile_expr(value, body);
                if let Expr::Variable(name) = &**target {
                    self.add_step(Instruction::NativeOp { 
                        name: "set_var".into(), 
                        args: vec![name.clone(), "stack".into()],
                        return_var: "void".into()
                    }, body);
                }
            }
            Expr::CompoundAssignment { target, op, value } => {
                self.compile_expr(value, body);
                if let Expr::Variable(name) = &**target {
                    self.add_step(Instruction::NativeOp { 
                        name: "compound_assign".into(), 
                        args: vec![name.clone(), op.lexeme.clone(), "stack".into()],
                        return_var: "void".into()
                    }, body);
                }
            }
            Expr::Ternary { cond, then, else_ } => {
                self.compile_expr(cond, body);
                self.compile_expr(then, body);
                self.compile_expr(else_, body);
                self.add_step(Instruction::NativeOp { 
                    name: "ternary".into(), 
                    args: vec!["stack_2".into(), "stack_1".into(), "stack_0".into()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg, body);
                }
                self.compile_expr(callee, body);
                self.add_step(Instruction::Call { func: "stack".into(), args: vec![] }, body);
            }
            Expr::New { class_name, args } => {
                for arg in args {
                    self.compile_expr(arg, body);
                }
                self.add_step(Instruction::NativeOp {
                    name: "new_instance".into(),
                    args: vec![class_name.clone()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Await(expr) => {
                self.compile_expr(expr, body);
                self.add_step(Instruction::Await { promise_var: "stack".into(), return_var: "stack".into() }, body);
            }
            Expr::Array(elements) => {
                for el in elements {
                    self.compile_expr(el, body);
                }
                self.add_step(Instruction::ArrayCreate { elements: vec![], return_var: "stack".into() }, body);
            }
            Expr::Object(fields) => {
                for (_name, val) in fields {
                    self.compile_expr(val, body);
                }
                self.add_step(Instruction::ObjectCreate { properties: vec![], return_var: "stack".into() }, body);
            }
            Expr::Grouping(inner) => {
                self.compile_expr(inner, body);
            }
            Expr::Match { cond, arms } => {
                self.compile_expr(cond, body);
                for (_pattern, guard, arm_expr) in arms {
                    if let Some(g) = guard { self.compile_expr(g, body); }
                    self.compile_expr(arm_expr, body);
                }
            }
            Expr::Get { object, name } => {
                self.compile_expr(object, body);
                self.add_step(Instruction::GetProp { object_var: "stack".into(), property_var: name.lexeme.clone(), return_var: "stack".into() }, body);
            }
            Expr::Set { object, name, value } => {
                self.compile_expr(value, body);
                self.compile_expr(object, body);
                self.add_step(Instruction::SetProp { object_var: "stack".into(), property_var: name.lexeme.clone(), value_var: "stack_1".into() }, body);
            }
            Expr::Greek(_) => {
                // Placeholder for Greek operations in IR
                self.add_step(Instruction::Log("Greek operation executed".into()), body);
            }
            Expr::CustomBinary { left, op, right } => {
                self.compile_expr(left, body);
                self.compile_expr(right, body);
                self.add_step(Instruction::NativeOp {
                    name: "custom_op".into(),
                    args: vec![op.lexeme.clone(), "stack_1".into(), "stack_0".into()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::UnitConversion { expr, target_unit } => {
                self.compile_expr(expr, body);
                self.add_step(Instruction::NativeOp {
                    name: "unit_convert".into(),
                    args: vec![target_unit.clone(), "stack".into()],
                    return_var: "stack".into()
                }, body);
            }
            Expr::Infer(name) => {
                self.add_step(Instruction::NativeOp {
                    name: "infer_type".into(),
                    args: vec![name.clone()],
                    return_var: "stack".into()
                }, body);
            }
        }
    }
}
