use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bootstrap_gen.rs");
    let mut f = File::create(&dest_path).unwrap();

    let mut scripts = Vec::new();
    let std_dir = Path::new("std");
    if !std_dir.exists() || !std_dir.is_dir() {
        panic!("'std' directory not found. This build script must be run from the project root.");
    }

    for entry in walkdir::WalkDir::new(std_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "ns") {
            let path = entry.path();
            let relative_path = path.strip_prefix(std_dir.parent().unwrap()).unwrap();
            let content = fs::read_to_string(path).unwrap();
            scripts.push((relative_path.to_str().unwrap().to_owned(), content));
        }
    }

    writeln!(f, "pub fn get_bootstrap_scripts() -> Vec<(&'static str, &'static str)> {{").unwrap();
    writeln!(f, "    vec![").unwrap();
    for (path, content) in &scripts {
        writeln!(
            f,
            "        (\"{}\", r####\"{}\"####),",
            path.replace('\\', "/"), 
            content.replace('\\', "\\\\")
        ).unwrap();
    }
    writeln!(f, "    ]").unwrap();
    writeln!(f, "}}").unwrap();
}
