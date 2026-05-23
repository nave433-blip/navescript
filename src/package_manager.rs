// src/package_manager.rs

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, Context};
use crate::{runtime, ns_lexer, ns_parser, ns_compiler, parser, ir, stdlib};
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub edition: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DependencyDetails),
}

#[derive(Deserialize, Debug)]
pub struct DependencyDetails {
    pub version: Option<String>,
    pub git: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFile {
    pub packages: HashMap<String, LockedPackage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockedPackage {
    pub version: String,
    pub source: String,
    pub commit: String,
}


pub fn install() -> Result<()> {
    let project_root = find_project_root()?;
    let manifest_path = project_root.join("nvs.toml");
    if !manifest_path.exists() {
        anyhow::bail!("no nvs.toml found in project root: {}", project_root.display());
    }

    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: Manifest = toml::from_str(&manifest_content)
        .with_context(|| "failed to parse nvs.toml")?;

    println!("Resolving dependencies for {}...", manifest.package.name);

    let mut installed = HashSet::new();
    let mut lock_packages = HashMap::new();
    
    resolve_dependencies(&project_root, &manifest, &mut installed, &mut lock_packages)?;

    let lock_file = LockFile { packages: lock_packages };
    let lock_content = toml::to_string_pretty(&lock_file)?;
    fs::write(project_root.join("nvs.lock"), lock_content)?;

    println!("Dependencies installed successfully and nvs.lock updated.");

    Ok(())
}

fn resolve_dependencies(
    project_root: &Path,
    manifest: &Manifest,
    installed: &mut HashSet<String>,
    lock_packages: &mut HashMap<String, LockedPackage>,
) -> Result<()> {
    let lib_dir = project_root.join("lib");
    if !lib_dir.exists() {
        fs::create_dir(&lib_dir)?;
    }

    for (name, dep) in &manifest.dependencies {
        if installed.contains(name) {
            continue;
        }

        match dep {
            Dependency::Detailed(details) => {
                if let Some(git_url) = &details.git {
                    println!("  Installing {} from {}", name, git_url);
                    let dep_path = lib_dir.join(name);
                    
                    if !dep_path.exists() {
                        let status = Command::new("git")
                            .arg("clone")
                            .arg(git_url)
                            .arg(&dep_path)
                            .status()?;
                        if !status.success() {
                            anyhow::bail!("failed to clone git repository for {}", name);
                        }
                    }

                    // Get current commit hash
                    let output = Command::new("git")
                        .arg("-C")
                        .arg(&dep_path)
                        .arg("rev-parse")
                        .arg("HEAD")
                        .output()?;
                    let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();

                    installed.insert(name.clone());
                    
                    // Recursive resolution
                    let dep_manifest_path = dep_path.join("nvs.toml");
                    let version = if dep_manifest_path.exists() {
                        let dep_manifest_content = fs::read_to_string(&dep_manifest_path)?;
                        let dep_manifest: Manifest = toml::from_str(&dep_manifest_content)?;
                        
                        // Recurse!
                        resolve_dependencies(project_root, &dep_manifest, installed, lock_packages)?;
                        
                        dep_manifest.package.version
                    } else {
                        "0.0.0".to_string()
                    };

                    lock_packages.insert(name.clone(), LockedPackage {
                        version,
                        source: git_url.clone(),
                        commit,
                    });
                }
            }
            _ => {
                println!("  Skipping dependency {}: only git dependencies are supported for now.", name);
            }
        }
    }
    Ok(())
}

pub fn find_project_root() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        if ancestor.join("nvs.toml").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }
    anyhow::bail!("not a Navescript project (or any of the parent directories): nvs.toml not found");
}

fn load_file_and_dependencies(
    file_path: &Path,
    project_root: &Path,
    loaded: &mut HashSet<PathBuf>,
) -> Result<String> {
    if !loaded.insert(file_path.to_path_buf()) {
        return Ok("".to_string()); // Already loaded
    }

    let source = fs::read_to_string(file_path)?;

    let mut all_source = String::new();
    for line in source.lines() {
        if line.trim().starts_with("import") {
            let parts: Vec<&str> = line.split(&[' ', ';', '"'][..]).collect();
            if parts.len() > 2 {
                let module_name = parts[2];
                let dep_path = project_root.join("lib").join(module_name).join("src").join("main.ns");
                if dep_path.exists() {
                    let dep_source = load_file_and_dependencies(&dep_path, project_root, loaded)?;
                    all_source.push_str(&dep_source);
                }
            }
        }
    }

    all_source.push_str(&source);
    Ok(all_source)
}

pub async fn run_project() -> Result<()> {
    let project_root = find_project_root()?;
    let main_file = project_root.join("src").join("main.ns");
    if !main_file.exists() {
        anyhow::bail!("src/main.ns not found in project");
    }

    let mut loaded_files = HashSet::new();
    let full_source = load_file_and_dependencies(&main_file, &project_root, &mut loaded_files)?;
    
    run_code(full_source, main_file.to_str().unwrap(), "permissive", 30000, 512).await
}

pub async fn test_project(filter: &Option<String>) -> Result<()> {
    let project_root = find_project_root()?;
    let tests_dir = project_root.join("tests");
    if !tests_dir.exists() {
        println!("No tests found.");
        return Ok(());
    }

    println!("Running tests...");

    let mut full_source = String::new();
    let mut loaded_files = HashSet::new();

    for entry in WalkDir::new(tests_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "ns") {
            if let Some(f) = filter {
                if !entry.path().to_str().unwrap().contains(f) {
                    continue;
                }
            }
            println!("  Testing {}", entry.path().display());
            let source = load_file_and_dependencies(entry.path(), &project_root, &mut loaded_files)?;
            full_source.push_str(&source);
        }
    }

    run_code(full_source, "tests.ns", "permissive", 30000, 512).await
}


// --- Private runner functions moved from runner.rs ---

pub async fn run_ns_script(runtime: &mut runtime::NaveRuntime, source: &str, _filename: &str) -> Result<()> {
    let ir = {
        let mut lexer = ns_lexer::Lexer::new(source.to_string());
        let tokens = lexer.scan_tokens().map_err(|e| anyhow::anyhow!(e))?;
        let mut parser = ns_parser::Parser::new(tokens);
        let stmts = parser.parse().map_err(|e| anyhow::anyhow!(e))?;
        let mut compiler = ns_compiler::TargetCompiler::new();
        compiler.compile(&stmts)
    };
    runtime.run_wasm_target(&ir).await?;
    Ok(())
}

pub async fn run_code(source: String, filename: &str, sandbox_mode: &str, _timeout_ms: u64, _memory_mb: u64) -> Result<()> {
    let sandbox = match sandbox_mode {
        "strict" => crate::Sandbox::strict(),
        "permissive" => crate::Sandbox::permissive(),
        _ => crate::Sandbox::permissive(),
    };

    let mut runtime = runtime::NaveRuntime::new(Some(sandbox))?;

    // Bootstrap the standard library
    for (name, script) in stdlib::bootstrap::get_bootstrap_scripts() {
        if let Err(e) = run_ns_script(&mut runtime, script, name).await {
            eprintln!("Failed to load bootstrap script {}: {}", name, e);
        }
    }

    let ir = if filename.ends_with(".ns") {
        let mut lexer = ns_lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens().map_err(|e| anyhow::anyhow!(e))?;
        let mut parser = ns_parser::Parser::new(tokens);
        let stmts = parser.parse().map_err(|e| anyhow::anyhow!(e))?;
        let mut compiler = ns_compiler::TargetCompiler::new();
        compiler.compile(&stmts)
    } else {
        let prog = parser::parse(&source)?;
        ir::NSIr::from_program(&prog).body
    };

    runtime.run_wasm_target(&ir).await?;
    Ok(())
}
