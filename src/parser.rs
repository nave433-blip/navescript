use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NaveProgram {
    pub version: String,
    pub module: String,
    pub world: Option<String>, // New: WASI Preview 2 World (e.g., "cli", "http")
    pub imports: Option<HashMap<String, String>>,
    pub capabilities: Option<Vec<String>>, // Granular capabilities (e.g., "filesystem/read")
    pub resources: Option<Vec<serde_json::Value>>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
    pub op: String,
    #[serde(flatten)]
    pub params: serde_json::Value,
}

pub fn parse(source: &str) -> Result<NaveProgram> {
    serde_json::from_str(source).map_err(|e| anyhow::anyhow!("Syntax Error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_program() {
        let json = r#"{
            "version": "0.3.0",
            "module": "test_module",
            "steps": [
                {
                    "op": "log",
                    "message": "test message"
                }
            ]
        }"#;

        let prog = parse(json).unwrap();
        assert_eq!(prog.version, "0.3.0");
        assert_eq!(prog.module, "test_module");
        assert_eq!(prog.steps.len(), 1);
        assert_eq!(prog.steps[0].op, "log");
        assert_eq!(prog.steps[0].params["message"], "test message");
    }

    #[test]
    fn test_parse_invalid_program() {
        let json = r#"{ "version": "0.3.0", "module": "test" "#; // Malformed JSON
        assert!(parse(json).is_err());
    }
}
