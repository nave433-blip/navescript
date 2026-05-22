use crate::runtime::{NaveRuntime, Value};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Static_scan_vulnerabilities", |args| async move {
        let code = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let mut findings = Vec::new();

        // Simple pattern-based "vulnerability" detection for demo
        if code.contains("eval(") {
            let mut vuln = HashMap::new();
            vuln.insert("type".to_string(), Value::String("Dangerous Sink".to_string()));
            vuln.insert("detail".to_string(), Value::String("Use of eval() detected".to_string()));
            findings.push(Value::Object(vuln));
        }

        if code.contains("memcpy") && !code.contains("sizeof") {
            let mut vuln = HashMap::new();
            vuln.insert("type".to_string(), Value::String("Potential Overflow".to_string()));
            vuln.insert("detail".to_string(), Value::String("memcpy called without dynamic size check".to_string()));
            findings.push(Value::Object(vuln));
        }

        Value::Array(findings)
    });
}
