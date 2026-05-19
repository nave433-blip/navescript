pub mod parser;
pub mod ir;
pub mod compiler;
pub mod runtime;
pub mod polyglot;
pub mod plugin;
pub mod nasm;
pub mod nasi;
pub mod ffi;
pub mod cranelift_backend;

pub use parser::NaveProgram;
pub use runtime::NaveRuntime;
