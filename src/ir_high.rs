use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::parser::NaveProgram;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NSIrHigh {
    pub module: String,
    pub intent: String,
    pub resources: Resources,
    pub steps: Vec<Step>,
    pub error_policy: Option<ErrorPolicy>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub target: String,        // "cpu", "gpu", "auto"
    pub parallel: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
    pub op: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorPolicy {
    pub on_error: String,      // "retry", "fallback", "panic"
    pub max_retries: u32,
    pub fallback_step: Option<String>,
}

impl NSIrHigh {
    pub fn from_program(prog: &NaveProgram) -> Self {
        let mut steps = Vec::new();
        for s in &prog.steps {
            let mut params = HashMap::new();
            if let Some(obj) = s.params.as_object() {
                for (k, v) in obj {
                    params.insert(k.clone(), v.to_string().replace('"', ""));
                }
            }
            steps.push(Step {
                op: s.op.clone(),
                params,
            });
        }

        let resources = if let Some(res) = &prog.resources {
             Resources {
                target: res.get("target").and_then(|v| v.as_str()).unwrap_or("auto").to_string(),
                parallel: res.get("parallel").and_then(|v| v.as_bool()).unwrap_or(false),
            }
        } else {
            Resources { target: "auto".to_string(), parallel: false }
        };

        Self {
            module: prog.module.clone().unwrap_or_else(|| "default".to_string()),
            intent: prog.intent.clone().unwrap_or_else(|| "none".to_string()),
            resources,
            steps,
            error_policy: None,
        }
    }
}
