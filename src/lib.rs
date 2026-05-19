pub mod parser;
pub mod ir;
pub mod compiler;
pub mod runtime;
pub mod polyglot;
pub mod plugin;

pub use parser::NaveProgram;
pub use runtime::NaveRuntime;
