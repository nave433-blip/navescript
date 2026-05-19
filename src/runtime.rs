use wasmtime::*;
use anyhow::Result;
use std::collections::HashMap;
use serde_json::Value;

use crate::ir::{NSIr, Instruction};
use crate::polyglot::PolyglotBridge;

pub struct NaveRuntime {
    _engine: Engine,
}

impl NaveRuntime {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let _engine = Engine::new(&config)?;
        Ok(Self { _engine })
    }

    pub fn interpret_ir(&self, ir: &NSIr) -> Result<()> {
        println!("🚀 Interpreting Nλvescript Core Engine...");
        let bridge = PolyglotBridge::new()?;
        let mut memory: HashMap<String, Value> = HashMap::new();

        for instr in &ir.body {
            match instr {
                Instruction::Log(msg) => {
                    println!("📝 Nλve Log: {}", msg);
                }
                Instruction::PolyglotEval { lang, code, input_var, return_var } => {
                    let input_json = if let Some(in_var) = input_var {
                        if let Some(val) = memory.get(in_var) {
                            Some(serde_json::to_string(val)?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let val = bridge.eval_and_transmute(lang, code, input_json)?;
                    memory.insert(return_var.clone(), val);
                }
                Instruction::Answer { var } => {
                    if let Some(val) = memory.get(var) {
                        println!("💬 Nλve Answer [{}]: {}", var, serde_json::to_string_pretty(val)?);
                    } else {
                        println!("⚠️ Nλve Error: Variable '{}' not found for answer.", var);
                    }
                }
                Instruction::SetVar { var, value } => {
                    memory.insert(var.clone(), value.clone());
                }
                Instruction::NativeOp { op, args, return_var } => {
                    let mut resolved_args = vec![];
                    for arg in args {
                        resolved_args.push(memory.get(arg).unwrap_or(&Value::Null).clone());
                    }
                    let res = match op.as_str() {
                        "add" => {
                            let a = resolved_args.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let b = resolved_args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            Value::from(a + b)
                        },
                        "concat" => {
                            let a = resolved_args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                            let b = resolved_args.get(1).and_then(|v| v.as_str()).unwrap_or("");
                            Value::from(format!("{}{}", a, b))
                        },
                        "multiply" => {
                            let a = resolved_args.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let b = resolved_args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            Value::from(a * b)
                        },
                        "reverse" => {
                            if let Some(arr) = resolved_args.get(0).and_then(|v| v.as_array()) {
                                let mut rev = arr.clone();
                                rev.reverse();
                                Value::Array(rev)
                            } else {
                                Value::Null
                            }
                        },
                        "uppercase" => {
                            if let Some(s) = resolved_args.get(0).and_then(|v| v.as_str()) {
                                Value::from(s.to_uppercase())
                            } else {
                                Value::Null
                            }
                        },
                        _ => Value::Null,
                    };
                    memory.insert(return_var.clone(), res);
                }
                Instruction::AssertEq { left_var, right_var, message } => {
                    let left = memory.get(left_var).unwrap_or(&Value::Null);
                    let right = memory.get(right_var).unwrap_or(&Value::Null);
                    
                    let mut is_equal = left == right;
                    if !is_equal && left.is_number() && right.is_number() {
                        if let (Some(l_num), Some(r_num)) = (left.as_f64(), right.as_f64()) {
                            is_equal = (l_num - r_num).abs() < f64::EPSILON;
                        }
                    }

                    if is_equal {
                        println!("✅ Match Verified [{} == {}]: {}", left_var, right_var, message);
                    } else {
                        println!("❌ Mismatch! {} ({}) != {} ({}). Context: {}", left_var, left, right_var, right, message);
                    }
                }
                _ => {}
            }
        }
        println!("✅ Execution complete.");
        Ok(())
    }
}