use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;
use navescript::runtime;
use navescript::parser;
use navescript::ir;
use navescript::ns_lexer;
use navescript::ns_parser;
use navescript::ns_compiler;
use navescript::polyglot;

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
    #[arg(short = 'V', long, global = true)]
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
    /// Execute a .ns or .nave file
    Run {
        file: PathBuf,
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
    /// Run test suite
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
    /// Install packages
    Install {
        package: String,
    },
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
            let source = fs::read_to_string(file)?;
            run_code(source, file.to_str().unwrap(), sandbox, *timeout, *memory_limit).await
        }
        Some(Commands::Eval { code }) => {
            run_code(code.clone(), "eval.ns", "permissive", 30000, 512).await
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
        Some(Commands::Repl) => {
            println!("Nλvescript v3.0.0 Interactive REPL");
            let mut rl = rustyline::DefaultEditor::new()?;
            loop {
                match rl.readline("nλ > ") {
                    Ok(line) => {
                        rl.add_history_entry(&line)?;
                        if line == "exit" || line == "quit" { break; }
                        let res = run_code(line, "repl.ns", "permissive", 30000, 512).await;
                        if let Err(e) = res { println!("Error: {}", e); }
                    }
                    Err(_) => break,
                }
            }
            Ok(())
        }
        Some(Commands::Fmt { write }) => {
            if let Some(file_path) = &cli.file {
                let source = fs::read_to_string(file_path)?;
                let prog = parser::parse(&source)?;
                let formatted = format!("{:#?}", prog);
                if *write {
                    fs::write(file_path, formatted)?;
                    println!("Formatted {}", file_path.display());
                } else {
                    println!("{}", formatted);
                }
            } else {
                println!("Please provide a file to format.");
            }
            Ok(())
        }
        Some(Commands::Lint { fix: _ }) => {
            if let Some(file_path) = &cli.file {
                let source = fs::read_to_string(file_path)?;
                let prog = parser::parse(&source)?;
                for stmt in &prog {
                    if let parser::Stmt::Fn { body, name, .. } = stmt {
                        if body.is_empty() {
                            println!("Warning: Function '{}' has an empty body.", name);
                        }
                    }
                }
            } else {
                println!("Please provide a file to lint.");
            }
            Ok(())
        }
        Some(Commands::Doc { serve: _ }) => {
            if let Some(file_path) = &cli.file {
                let source = fs::read_to_string(file_path)?;
                let prog = parser::parse(&source)?;
                println!("Documentation for {}:", file_path.display());
                for stmt in &prog {
                    if let parser::Stmt::Fn { name, .. } = stmt {
                        println!(" - Function: {}", name);
                    }
                }
            } else {
                println!("Please provide a file to generate docs for.");
            }
            Ok(())
        }
        Some(Commands::Init { name }) => {
            println!("Initializing project: {}", name);
            Ok(())
        }
        Some(Commands::Install { package }) => {
            println!("Installing package: {}", package);
            Ok(())
        }
        Some(Commands::Publish { .. }) => {
            println!("Publishing package...");
            Ok(())
        }
        Some(Commands::Clean) => {
            println!("Cleaning build artifacts...");
            Ok(())
        }
        Some(Commands::Test { filter }) => {
            println!("Running tests (filter: {:?})...", filter);
            Ok(())
        }
        Some(Commands::Lsp) => {
            println!("Starting LSP server...");
            Ok(())
        }
        None => {
            if let Some(file_path) = &cli.file {
                let source = fs::read_to_string(file_path)
                    .with_context(|| format!("Failed to read file: {:?}", file_path))?;
                run_code(source, file_path.to_str().unwrap(), "permissive", 30000, 512).await
            } else {
                println!("Nλvescript v3.0.0");
                println!("Usage: navescript <command> [options] or navescript <file.ns>");
                Ok(())
            }
        }
    }
}

async fn run_code(source: String, filename: &str, sandbox_mode: &str, _timeout_ms: u64, _memory_mb: u64) -> Result<()> {
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

    let sandbox = match sandbox_mode {
        "strict" => navescript::Sandbox::strict(),
        "permissive" => navescript::Sandbox::permissive(),
        _ => navescript::Sandbox::permissive(),
    };

    let mut runtime = runtime::NaveRuntime::new(Some(sandbox))?;
    runtime.run_wasm_target(&ir).await?;
    Ok(())
}
