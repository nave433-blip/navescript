mod parser;
mod ir;
mod compiler;
mod runtime;
mod polyglot;
mod plugin;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs;

#[derive(Parser)]
#[command(name = "nλvescript")]
#[command(author = "Evan Shipley")]
#[command(version = "0.3.0")]
#[command(about = "The Nλvescript Universal Polyglot Language & Runtime", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a .nave script directly
    Run { 
        #[arg(value_name = "FILE")]
        file: String 
    },
    /// Compile a .nave script to WASM/WAT
    Compile { 
        #[arg(value_name = "FILE")]
        file: String,
        #[arg(short, long, default_value = "output.wat")]
        output: String
    },
    /// Show current polyglot bridge status
    Status,
    /// List available plugins
    Plugins,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let source = fs::read_to_string(&file)?;
            let prog = parser::parse(&source)?;
            let ir = ir::NSIr::from_program(&prog);

            let runtime = runtime::NaveRuntime::new()?;
            runtime.interpret_ir(&ir)?;
        }
        Commands::Compile { file, output } => {
            let source = fs::read_to_string(&file)?;
            let output_base = output.strip_suffix(".wat").unwrap_or(&output);
            compiler::full_build(&source, output_base)?;
        }
        Commands::Status => {
            polyglot::show_status()?;
        }
        Commands::Plugins => {
            let manager = plugin::PluginManager::new("plugins");
            manager.discover();
        }
    }

    Ok(())
}
