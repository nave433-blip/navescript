pub mod binary;
pub mod process;
pub mod network;
pub mod fuzzer;
pub mod crypto_analyzer;
pub mod static_analyzer;
pub mod debugger;
pub mod forensics;
pub mod taint;
pub mod safeguards;

pub fn register(rt: &mut crate::runtime::NaveRuntime) {
    binary::register(rt);
    process::register(rt);
    network::register(rt);
    fuzzer::register(rt);
    crypto_analyzer::register(rt);
    static_analyzer::register(rt);
    debugger::register(rt);
    forensics::register(rt);
    taint::register(rt);
    safeguards::register(rt);
}
