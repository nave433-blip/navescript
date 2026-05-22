// src/package.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

pub fn init_project() -> Result<()> {
    let manifest = Manifest {
        name: "my_navescript_project".to_string(),
        version: "0.1.0".to_string(),
        description: Some("A new Navescript project".to_string()),
        main: Some("src/main.ns".to_string()),
        dependencies: Some(HashMap::new()),
    };

    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write("navescript.json", json)?;
    fs::create_dir_all("src")?;
    fs::write("src/main.ns", "fn main() {\n    log(\"Hello, Navescript!\");\n}\n")?;
    
    Ok(())
}

pub async fn install_package(package_name: &str) -> Result<()> {
    println!("Installing package {} from registry...", package_name);
    // TODO: Implement actual registry fetching
    Ok(())
}
