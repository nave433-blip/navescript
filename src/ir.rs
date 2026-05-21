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
    pub nit: Option<crate::parser::NitWorld>,
    pub imports: Vec<String>,
    pub requirements: Vec<Capability>,
    pub resources: Vec<Resource>,
    pub body: Vec<Instruction>,
    pub exports: Vec<String>,
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
    Load { reg: u8, addr: u32 },
    LoadImm { reg: u8, value: i32 },
    Store { reg: u8, addr: u32 },
    Add { dest: u8, src1: u8, src2: u8 },
    Ret { reg: u8 },
    Syscall { id: u32, args: Vec<u8> },
    HttpGet { url_var: String, return_var: String },
    HttpPost { url_var: String, body_var: String, return_var: String },
    ShellExec { command_var: String, return_var: String },
    FileRead { path_var: String, return_var: String },
    FileWrite { path_var: String, data_var: String },
    FsRemove { path_var: String, return_var: String },
    Sleep { ms_var: String },
    If { condition_var: String, then_body: Vec<Instruction>, else_body: Option<Vec<Instruction>> },
    ForEach { array_var: String, item_var: String, body: Vec<Instruction> },
    Input { prompt_var: String, return_var: String },
    JsonParse { string_var: String, return_var: String },
    ComponentCall { component_path_var: String, func_var: String, args_var: String, return_var: String },
}

impl NSIr {
    pub fn from_program(prog: &crate::parser::NaveProgram) -> Self {
        let body = Self::parse_steps(&prog.steps);

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
                        methods: vec![], 
                    });
                }
            }
        }

        Self {
            module_name: prog.module.clone(),
            world: prog.world.clone().unwrap_or_else(|| "cli".to_string()),
            nit: prog.nit.clone(),
            imports: prog.imports.clone().unwrap_or_default(),
            requirements,
            resources,
            body,
            exports: prog.exports.clone().unwrap_or_default(),
        }
    }

    fn parse_steps(steps: &[crate::parser::Step]) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        for s in steps {
            let ret_var = s.returns.clone()
                .or_else(|| s.params.get("return_var").and_then(|v| v.as_str()).map(|s| s.to_string()))
                .unwrap_or_else(|| "result".to_string());
            
            // Extract from 'params' if exists, otherwise assume flat structure for backward compat
            let get_param = |key: &str| -> String {
                s.params.get(key).and_then(|v| v.as_str())
                    .or_else(|| s.params.get("params").and_then(|p| p.get(key)).and_then(|v| v.as_str()))
                    .unwrap_or("")
                    .to_string()
            };

            let base_instr = match s.op.as_str() {
                "log" => Instruction::Log(get_param("message")),
                "polyglot_eval" => {
                    let code = s.params.get("code").and_then(|v| v.as_str())
                        .or_else(|| s.params.get("params").and_then(|p| p.get("code")).and_then(|v| v.as_str()))
                        .unwrap_or("")
                        .to_string();
                    Instruction::PolyglotEval {
                        lang: s.lang.clone().unwrap_or_else(|| s.params.get("lang").and_then(|v| v.as_str()).unwrap_or("python").to_string()),
                        code,
                        input_var: s.input.clone(),
                        return_var: ret_var,
                    }
                },
                "nasm_exec" => {
                    let code = s.params.get("code").and_then(|v| v.as_str())
                        .or_else(|| s.params.get("params").and_then(|p| p.get("code")).and_then(|v| v.as_str()))
                        .unwrap_or("")
                        .to_string();
                    Instruction::PolyglotEval {
                        lang: "nasm".to_string(),
                        code,
                        input_var: None,
                        return_var: ret_var,
                    }
                },
                "answer" => Instruction::Answer {
                    var: s.params.get("var").and_then(|v| v.as_str())
                        .or_else(|| s.params.get("params").and_then(|p| p.get("var")).and_then(|v| v.as_str()))
                        .unwrap_or("result")
                        .to_string(),
                },
                "set" => Instruction::SetVar {
                    var: s.params.get("var").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    value: s.params.get("value").unwrap_or(&serde_json::Value::Null).clone(),
                },
                "http_get" => Instruction::HttpGet {
                    url_var: get_param("url"),
                    return_var: ret_var,
                },
                "file_read" => Instruction::FileRead {
                    path_var: get_param("path"),
                    return_var: ret_var,
                },
                "file_write" => Instruction::FileWrite {
                    path_var: get_param("path"),
                    data_var: get_param("data"),
                },
                "input" => Instruction::Input {
                    prompt_var: get_param("prompt"),
                    return_var: ret_var,
                },
                "component_call" => Instruction::ComponentCall {
                    component_path_var: get_param("component"),
                    func_var: get_param("func"),
                    args_var: s.input.clone().unwrap_or_default(),
                    return_var: ret_var,
                },
                "try" => {
                    let try_body = s.params.get("body").and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Self::parse_steps(&steps)
                        }).unwrap_or_default();
                    let catch_body = s.params.get("catch").and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Self::parse_steps(&steps)
                        }).unwrap_or_default();
                    Instruction::Try {
                        body: try_body,
                        catch: catch_body,
                        retry_count: s.params.get("retry").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                    }
                },
                "native_op" => {
                    let args = s.params.get("args").and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_else(|| vec![]);
                    Instruction::NativeOp {
                        op: s.params.get("operator").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        args,
                        return_var: ret_var,
                    }
                },
                "if" => {
                    let condition = s.params.get("condition").and_then(|v| v.as_str())
                        .or_else(|| s.params.get("params").and_then(|p| p.get("condition")).and_then(|v| v.as_str()))
                        .unwrap_or("")
                        .to_string();
                    
                    let then_val = s.params.get("then").or_else(|| s.params.get("params").and_then(|p| p.get("then")));
                    let else_val = s.params.get("else").or_else(|| s.params.get("params").and_then(|p| p.get("else")));

                    let then_body = then_val.and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Self::parse_steps(&steps)
                        }).unwrap_or_default();
                    let else_body = else_val.and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Some(Self::parse_steps(&steps))
                        }).unwrap_or(None);
                    Instruction::If {
                        condition_var: condition,
                        then_body,
                        else_body,
                    }
                }
                _ => Instruction::Log(format!("Unknown op: {}", s.op)),
            };

            // Handle retry/on_error by wrapping in Try if needed
            if let Some(rc) = s.retry {
                instrs.push(Instruction::Try {
                    body: vec![base_instr],
                    catch: vec![], // Future: add on_error handling here
                    retry_count: rc,
                });
            } else {
                instrs.push(base_instr);
            }
        }
        instrs
    }
}
