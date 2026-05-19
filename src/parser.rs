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
