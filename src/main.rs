use clap::{Parser as ClapParser, Subcommand};
use anyhow::{Result};
use std::fs;
use std::path::PathBuf;
use navescript::parser;
use navescript::ir;
use navescript::polyglot;
use navescript::package_manager;

#[derive(ClapParser)]
#[command(name = "navescript")]
#[command(version = "3.0.0")]
#[command(about = "Nλvescript v3.0: Universal Polyglot Runtime", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the .nave or .ns file to run
    file: Option<PathBuf>,

    /// Verbose output
    #[arg(short = 'v', long, global = true)]
    verbose: bool,

    /// Suppress non-error output
    #[arg(short = 'q', long, global = true)]
    quiet: bool,

    /// Force color output
    #[arg(long, global = true, default_value = "auto")]
    color: String,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    /// Set log level
    #[arg(long, global = true, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a .ns or .nave file, or run the current project
    Run {
        #[arg()]
        file: Option<PathBuf>,
        /// Sandbox mode
        #[arg(long, default_value = "permissive")]
        sandbox: String,
        /// Execution timeout (ms)
        #[arg(long, default_value_t = 30000)]
        timeout: u64,
        /// Memory limit (MB)
        #[arg(long, default_value_t = 512)]
        memory_limit: u64,
    },
    /// Execute code directly from string
    Eval {
        code: String,
    },
    /// Start interactive REPL
    Repl,
    /// Launch the NvS CodeCamp
    Camp,
    /// Launch the Navescript Package Manager
    Pm {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run test suite for the current project
    Test {
        #[arg(long)]
        filter: Option<String>,
    },
    /// Format code
    Fmt {
        #[arg(long)]
        write: bool,
    },
    /// Lint code
    Lint {
        #[arg(long)]
        fix: bool,
    },
    /// Generate documentation
    Doc {
        #[arg(long)]
        serve: bool,
    },
    /// Compile a Navescript file to IR
    Compile {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Show current polyglot bridge status
    Polyglot,
    /// Start LSP server
    Lsp,
    /// Create new project
    Init {
        name: String,
    },
    /// Install packages from nvs.toml
    Install,
    /// Publish package
    Publish {
        #[arg(long)]
        tag: Option<String>,
    },
    /// Remove build artifacts
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file, sandbox, timeout, memory_limit }) => {
            if let Some(file_path) = file {
                let source = fs::read_to_string(file_path)?;
                package_manager::run_code(source, file_path.to_str().unwrap(), sandbox, *timeout, *memory_limit).await
            } else {
                package_manager::run_project().await
            }
        }
        Some(Commands::Eval { code }) => {
            package_manager::run_code(code.clone(), "eval.ns", "permissive", 30000, 512).await
        }
        Some(Commands::Test { filter }) => {
            package_manager::test_project(filter).await
        }
        Some(Commands::Repl) => {
            println!("Nλvescript v3.0.0 Interactive REPL");
            let mut rl = rustyline::DefaultEditor::new()?;
            loop {
                match rl.readline("nλ > ") {
                    Ok(line) => {
                        let _ = rl.add_history_entry(&line);
                        if line == "exit" || line == "quit" { break; }
                        let res = package_manager::run_code(line, "repl.ns", "permissive", 30000, 512).await;
                        if let Err(e) = res { println!("Error: {}", e); }
                    }
                    Err(_) => break,
                }
            }
            Ok(())
        }
        Some(Commands::Camp) => {
            let code = "import std.cli.codecamp; codecamp.run_camp();".to_string();
            package_manager::run_code(code, "camp.ns", "permissive", 0, 0).await
        }
        Some(Commands::Pm { args }) => {
            let code = format!("import std.pm.nvspm; nvspm.setup_cli({:?});", args);
            package_manager::run_code(code, "pm.ns", "permissive", 0, 0).await
        }
        Some(Commands::Init { name }) => {
            let project_path = PathBuf::from(name);
            if project_path.exists() {
                anyhow::bail!("destination `{}` already exists", name);
            }

            println!("Creating new Navescript project: {}", name);

            fs::create_dir_all(&project_path)?;
            let src_path = project_path.join("src");
            fs::create_dir(&src_path)?;
            let tests_path = project_path.join("tests");
            fs::create_dir(&tests_path)?;

            let nvs_toml_content = format!(
                r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
[dependencies]
"#,
                name
            );
            fs::write(project_path.join("nvs.toml"), nvs_toml_content)?;

            let main_ns_content = "func main() {\n    console.log(\"Hello, world!\");\n}\n";
            fs::write(src_path.join("main.ns"), main_ns_content)?;
            
            let test_main_ns_content = r#"// tests/main.ns
import std.test.testing;
test("example test", () => {
    assert_eq(1 + 1, 2);
});
"#;
            fs::write(tests_path.join("main.ns"), test_main_ns_content)?;

            let gitignore_content = "/target\n/lib\nnvs.lock\n";
            fs::write(project_path.join(".gitignore"), gitignore_content)?;

            println!("Project `{}` created successfully.", name);

            Ok(())
        }
        Some(Commands::Install) => {
            package_manager::install()
        }
        Some(Commands::Compile { file, output }) => {
            let source = fs::read_to_string(file)?;
            let prog = parser::parse(&source)?;
            let ir = ir::NSIr::from_program(&prog);
            let json = serde_json::to_string_pretty(&ir)?;

            if let Some(out_path) = output {
                fs::write(out_path, json)?;
            } else {
                println!("{}", json);
            }
            Ok(())
        }
        Some(Commands::Polyglot) => {
            polyglot::show_status()?;
            Ok(())
        }
        Some(Commands::Fmt { write }) => {
            if let Some(file_path) = &cli.file {
                let code = format!("import std.fmt.formatter; formatter.format({:?}, {});", file_path.to_str().unwrap(), write);
                package_manager::run_code(code, "fmt_gen.ns", "permissive", 0, 0).await
            } else {
                println!("Please provide a file to format.");
                Ok(())
            }
        }
        Some(Commands::Lint { fix: _ }) => {
            if let Some(file_path) = &cli.file {
                let code = format!("import std.lint.checker; checker.lint({:?});", file_path.to_str().unwrap());
                package_manager::run_code(code, "lint_gen.ns", "permissive", 0, 0).await
            } else {
                println!("Please provide a file to lint.");
                Ok(())
            }
        }
        Some(Commands::Doc { serve: _ }) => {
            if let Some(file_path) = &cli.file {
                let code = format!("import std.doc.gen; gen.generate_docs({:?});", file_path.to_str().unwrap());
                package_manager::run_code(code, "doc_gen.ns", "permissive", 0, 0).await
            } else {
                println!("Please provide a file to generate docs for.");
                Ok(())
            }
        }
        Some(Commands::Publish { .. }) => {
            println!("Not yet implemented");
            Ok(())
        }
        Some(Commands::Clean) => {
            println!("Cleaning build artifacts...");
            let project_root = package_manager::find_project_root()?;
            let target = project_root.join("target");
            let lib = project_root.join("lib");
            if target.exists() {
                fs::remove_dir_all(&target)?;
                println!("  Removed target/");
            }
            if lib.exists() {
                fs::remove_dir_all(&lib)?;
                println!("  Removed lib/");
            }
            println!("✅ Project cleaned.");
            Ok(())
        }
        Some(Commands::Lsp) => {
            println!("Not yet implemented");
            Ok(())
        }
        None => {
            if let Some(file_path) = &cli.file {
                let source = fs::read_to_string(file_path)?;
                package_manager::run_code(source, file_path.to_str().unwrap(), "permissive", 30000, 512).await
            } else {
                println!("Nλvescript v3.0.0");
                println!("Usage: navescript <command> [options] or navescript <file.ns>");
                Ok(())
            }
        }
    }
}
