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
pub struct StepIr {
    pub id: Option<String>,
    pub instr: Instruction,
    pub next: Option<String>,
    pub on_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NSIr {
    pub module_name: String,
    pub world: String,
    pub nit: Option<crate::parser::NitWorld>,
    pub imports: Vec<String>,
    pub requirements: Vec<Capability>,
    pub resources: Vec<Resource>,
    pub body: Vec<StepIr>,
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
        body: Vec<StepIr>,
        catch: Vec<StepIr>,
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
        name: String,
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
    Sub { dest: u8, src1: u8, src2: u8 },
    Mul { dest: u8, src1: u8, src2: u8 },
    Div { dest: u8, src1: u8, src2: u8 },
    Mod { dest: u8, src1: u8, src2: u8 },
    And { dest: u8, src1: u8, src2: u8 },
    Or { dest: u8, src1: u8, src2: u8 },
    Xor { dest: u8, src1: u8, src2: u8 },
    Shl { dest: u8, src1: u8, src2: u8 },
    Shr { dest: u8, src1: u8, src2: u8 },
    Not { dest: u8, src: u8 },
    Jmp { label: String },
    Jz { reg: u8, label: String },
    Jnz { reg: u8, label: String },
    Cmp { left: u8, right: u8 },
    Ret { reg: u8 },
    Return { value_var: Option<String> },
    Push { value: serde_json::Value },
    Pop { var: String },
    CallLabel { label: String },
    Alloc { size: u32, return_reg: u8 },
    Syscall { id: u32, args: Vec<u8> },
    HttpGet { url_var: String, return_var: String },
    HttpPost { url_var: String, body_var: String, return_var: String },
    ShellExec { command_var: String, return_var: String },
    FileRead { path_var: String, return_var: String },
    FileWrite { path_var: String, data_var: String },
    FsRemove { path_var: String, return_var: String },
    Sleep { ms_var: String },
    If { condition_var: String, then_body: Vec<StepIr>, else_body: Option<Vec<StepIr>> },
    While { condition: Vec<StepIr>, condition_var: String, body: Vec<StepIr> },
    ForEach { array_var: String, item_var: String, body: Vec<StepIr> },
    DefineFunc { name: String, params: Vec<String>, body: Vec<StepIr> },
    Block(Vec<StepIr>),
    Input { prompt_var: String, return_var: String },
    JsonParse { string_var: String, return_var: String },
    ComponentCall { component_path_var: String, func_var: String, args_var: String, return_var: String },
    Await { promise_var: String, return_var: String },
    Throw { error_var: String },
    ArrayCreate { elements: Vec<String>, return_var: String },
    ObjectCreate { properties: Vec<(String, String)>, return_var: String },
    GetProp { object_var: String, property_var: String, return_var: String },
    SetProp { object_var: String, property_var: String, value_var: String },
    Yield { value_var: String },
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

    fn parse_steps(steps: &[crate::parser::Step]) -> Vec<StepIr> {
        let mut instrs = Vec::new();
        for s in steps {
            let ret_var = s.returns.clone()
                .or_else(|| s.params.get("return_var").and_then(|v| v.as_str()).map(|s| s.to_string()))
                .unwrap_or_else(|| "result".to_string());
            
            // Helper for extracting parameters from flat or nested 'params'
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
                    let input_var = s.input.clone()
                        .or_else(|| s.params.get("input_var").and_then(|v| v.as_str()).map(|s| s.to_string()))
                        .or_else(|| s.params.get("input").and_then(|v| v.as_str()).map(|s| s.to_string()));
                    Instruction::PolyglotEval {
                        lang: s.lang.clone().unwrap_or_else(|| s.params.get("lang").and_then(|v| v.as_str()).unwrap_or("python").to_string()),
                        code,
                        input_var,
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
                "native_op" => Instruction::NativeOp {
                    name: get_param("name"),
                    args: s.params.get("args").and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default(),
                    return_var: ret_var,
                },
                "assert_eq" => Instruction::AssertEq {
                    left_var: get_param("left_var"),
                    right_var: get_param("right_var"),
                    message: get_param("message"),
                },
                "http_get" => Instruction::HttpGet {
                    url_var: get_param("url"),
                    return_var: ret_var,
                },
                "http_post" => Instruction::HttpPost {
                    url_var: get_param("url"),
                    body_var: get_param("body"),
                    return_var: ret_var,
                },
                "shell_exec" => Instruction::ShellExec {
                    command_var: get_param("command"),
                    return_var: ret_var,
                },
                "json_parse" => Instruction::JsonParse {
                    string_var: get_param("json"),
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
                "for_each" => {
                    let body_val = s.params.get("body").or_else(|| s.params.get("params").and_then(|p| p.get("body")));
                    let body = body_val.and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Self::parse_steps(&steps)
                        }).unwrap_or_default();
                    Instruction::ForEach {
                        array_var: get_param("array"),
                        item_var: get_param("item"),
                        body,
                    }
                },
                "component_call" => Instruction::ComponentCall {
                    component_path_var: get_param("component"),
                    func_var: get_param("func"),
                    args_var: s.input.clone().unwrap_or_default(),
                    return_var: ret_var,
                },
                "try" => {
                    let try_body_val = s.params.get("body").or_else(|| s.params.get("params").and_then(|p| p.get("body")));
                    let catch_body_val = s.params.get("catch").or_else(|| s.params.get("params").and_then(|p| p.get("catch")));

                    let try_body = try_body_val.and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Self::parse_steps(&steps)
                        }).unwrap_or_default();
                    let catch_body = catch_body_val.and_then(|v| v.as_array())
                        .map(|arr| {
                            let steps: Vec<crate::parser::Step> = serde_json::from_value(serde_json::Value::Array(arr.clone())).unwrap_or_default();
                            Self::parse_steps(&steps)
                        }).unwrap_or_default();
                    Instruction::Try {
                        body: try_body,
                        catch: catch_body,
                        retry_count: s.retry.unwrap_or(0),
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

            instrs.push(StepIr {
                id: s.id.clone(),
                instr: base_instr,
                next: s.next.clone(),
                on_error: s.on_error.clone(),
            });
        }
        instrs
    }
}
