// src/stdlib/bootstrap.rs

// This file contains the Navescript standard library source files,
// embedded directly into the executable for portability.

const NET_HTTP: &str = include_str!("../../../std/net/http.ns");
const COMPILER_PARSER: &str = include_str!("../../../std/compiler/parser.ns");
const COMPILER_AST: &str = include_str!("../../../std/compiler/ast.ns");
const TEST_TESTING: &str = include_str!("../../../std/test/testing.ns");
const COMPILER_LEXER: &str = include_str!("../../../std/compiler/lexer.ns");
const RPC_JSONRPC: &str = include_str!("../../../std/rpc/jsonrpc.ns");
const CONFIG_VIPER: &str = include_str!("../../../std/config/viper.ns");
const CLI_CMD: &str = include_str!("../../../std/cli/cmd.ns");
const CORE_BASE: &str = include_str!("../../../std/core/base.ns");

/// Returns a list of all standard library scripts.
/// The tuple contains (path, content).
pub fn get_bootstrap_scripts() -> Vec<(&'static str, &'static str)> {
    vec![
        ("std/net/http.ns", NET_HTTP),
        ("std/compiler/parser.ns", COMPILER_PARSER),
        ("std/compiler/ast.ns", COMPILER_AST),
        ("std/test/testing.ns", TEST_TESTING),
        ("std/compiler/lexer.ns", COMPILER_LEXER),
        ("std/rpc/jsonrpc.ns", RPC_JSONRPC),
        ("std/config/viper.ns", CONFIG_VIPER),
        ("std/cli/cmd.ns", CLI_CMD),
        ("std/core/base.ns", CORE_BASE),
    ]
}
