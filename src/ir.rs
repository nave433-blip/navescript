use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Ownership {
    Owned,
    Borrowed,
    Shared,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Method {
    pub name: String,
    pub params: String,
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    pub name: String,
    pub ownership: Ownership,
    pub methods: Vec<Method>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NSIr {
    pub module_name: String,
    pub world: String,
    pub imports: Vec<Import>,
    pub requirements: Vec<Capability>,
    pub resources: Vec<Resource>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Import {
    Wasi { namespace: String, name: String },
    Polyglot { lang: String, module: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Capability {
    FsRead(String),
    FsWrite(String),
    NetConnect(String),
    Random,
    ClockMonotonic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Instruction {
    Log(String),
    Call { func: String, args: Vec<String> },
    Try {
        body: Vec<Instruction>,
        catch: Vec<Instruction>,
        retry_count: u32,
    },
    PolyglotEval {
        lang: String,
        code: String,
        input_var: Option<String>,
        return_var: String,
    },
    Answer {
        var: String,
    },
    SetVar {
        var: String,
        value: serde_json::Value,
    },
    NativeOp {
        op: String,
        args: Vec<String>,
        return_var: String,
    },
    AssertEq {
        left_var: String,
        right_var: String,
        message: String,
    },
}

impl NSIr {
    pub fn from_program(prog: &crate::parser::NaveProgram) -> Self {
        let mut body = Vec::new();
        for s in &prog.steps {
            match s.op.as_str() {
                "log" => body.push(Instruction::Log(s.params.get("message").and_then(|v| v.as_str()).unwrap_or("").to_string())),
                "call" => body.push(Instruction::Call {
                    func: s.params.get("func").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    args: vec![],
                }),
                "polyglot_eval" => body.push(Instruction::PolyglotEval {
                    lang: s.params.get("lang").and_then(|v| v.as_str()).unwrap_or("python").to_string(),
                    code: s.params.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    input_var: s.params.get("input_var").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    return_var: s.params.get("return_var").and_then(|v| v.as_str()).unwrap_or("result").to_string(),
                }),
                "answer" => body.push(Instruction::Answer {
                    var: s.params.get("var").and_then(|v| v.as_str()).unwrap_or("result").to_string(),
                }),
                "set" => body.push(Instruction::SetVar {
                    var: s.params.get("var").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    value: s.params.get("value").unwrap_or(&serde_json::Value::Null).clone(),
                }),
                "native_op" => {
                    let args = s.params.get("args").and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_else(|| vec![]);
                    body.push(Instruction::NativeOp {
                        op: s.params.get("operator").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        args,
                        return_var: s.params.get("return_var").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    });
                },
                "assert_eq" => body.push(Instruction::AssertEq {
                    left_var: s.params.get("left").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    right_var: s.params.get("right").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    message: s.params.get("message").and_then(|v| v.as_str()).unwrap_or("Assertion failed").to_string(),
                }),
                "try" => {
                    body.push(Instruction::Try {
                        body: vec![], 
                        catch: vec![],
                        retry_count: s.params.get("retry").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                    });
                }
                _ => {}
            }
        }

        let mut requirements = Vec::new();
        if let Some(caps) = &prog.capabilities {
            for cap in caps {
                match cap.as_str() {
                    "filesystem/read" => requirements.push(Capability::FsRead(".".to_string())),
                    "filesystem/write" => requirements.push(Capability::FsWrite(".".to_string())),
                    "random" => requirements.push(Capability::Random),
                    _ => {}
                }
            }
        }
        
        let mut resources = Vec::new();
        if let Some(prog_res) = &prog.resources {
            for res_val in prog_res {
                if let Some(obj) = res_val.as_object() {
                    let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                    let ownership_str = obj.get("ownership").and_then(|v| v.as_str()).unwrap_or("owned");
                    let ownership = match ownership_str {
                        "borrowed" => Ownership::Borrowed,
                        "shared" => Ownership::Shared,
                        _ => Ownership::Owned,
                    };
                    resources.push(Resource {
                        name,
                        ownership,
                        methods: vec![], // Expand parsing as needed
                    });
                }
            }
        }

        Self {
            module_name: prog.module.clone(),
            world: prog.world.clone().unwrap_or_else(|| "cli".to_string()),
            imports: vec![],
            requirements,
            resources,
            body,
        }
    }
}
