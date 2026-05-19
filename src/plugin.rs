use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub entry_point: String,
    pub language: String, // "go", "perl", "rust", "swift", "java", "html"
}

pub struct PluginManager {
    pub plugins_path: String,
}

impl PluginManager {
    pub fn new(path: &str) -> Self {
        Self { plugins_path: path.to_string() }
    }

    pub fn discover(&self) -> Vec<PluginMetadata> {
        println!("🔍 Discovering universal plugins in {}...", self.plugins_path);
        // This would scan for .so, .dylib, .wasm, .jar, .html etc.
        vec![]
    }
}

// C-ABI FFI Header for Plugin Integration
#[repr(C)]
pub struct NavePluginAPI {
    pub name: *const i8,
    pub execute: extern "C" fn(*const i8) -> *const i8,
    pub get_version: extern "C" fn() -> *const i8,
}
