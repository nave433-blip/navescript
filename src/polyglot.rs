use anyhow::Result;
use std::process::Command;
use serde_json::Value;

pub struct PolyglotBridge;

impl PolyglotBridge {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn show_status(&self) -> Result<()> {
        println!("🌐 GraalVM Polyglot Bridge Status:");
        println!("   • Host Runtime: Rust (Nλvescript Core)");
        println!("   • Guest Target: Python, JS, Ruby, Perl, Bash");
        println!("   • Interop Protocols: JSON Transmutation & ENV Injection");
        println!("   • Transmutation Layer: Active");
        Ok(())
    }

    /// Evaluates foreign code and transmutates the result into a Nλvescript native value.
    pub fn eval_and_transmute(&self, lang: &str, code: &str, input_json: Option<String>) -> Result<Value> {
        println!("🔄 [Transmuter] Executing {} code...", lang);
        
        let mut cmd = match lang {
            "python" => {
                let wrapper = format!(
                    "import json\nimport os\ninput_data = json.loads(os.environ.get('NAVE_INPUT') or 'null')\n{}\nif 'result' in locals():\n    print(json.dumps(result))\nelse:\n    print('null')", 
                    code
                );
                let mut c = Command::new("python3");
                c.arg("-c").arg(&wrapper);
                c
            },
            "js" | "javascript" => {
                let wrapper = format!(
                    "const input_data = JSON.parse(process.env.NAVE_INPUT || 'null');\n{}\nif (typeof result !== 'undefined') console.log(JSON.stringify(result)); else console.log('null');",
                    code
                );
                let mut c = Command::new("node");
                c.arg("-e").arg(&wrapper);
                c
            },
            "ruby" => {
                let wrapper = format!(
                    "require 'json'\ninput_data = JSON.parse(ENV['NAVE_INPUT'] || 'null')\n{}\nputs(defined?(result) ? result.to_json : 'null')",
                    code
                );
                let mut c = Command::new("ruby");
                c.arg("-e").arg(&wrapper);
                c
            },
            "perl" => {
                let wrapper = format!(
                    "use JSON::PP;\nmy $input_data = decode_json($ENV{{NAVE_INPUT}} || 'null');\n{}\nprint defined($result) ? encode_json($result) : 'null';",
                    code
                );
                let mut c = Command::new("perl");
                c.arg("-e").arg(&wrapper);
                c
            },
            "bash" | "sh" => {
                let wrapper = format!(
                    "input_data=\"$NAVE_INPUT\"\n{}\nif [ -n \"$result\" ]; then echo \"$result\"; else echo 'null'; fi",
                    code
                );
                let mut c = Command::new("bash");
                c.arg("-c").arg(&wrapper);
                c
            },
            _ => anyhow::bail!("Unsupported polyglot language: {}", lang),
        };

        if let Some(json) = input_json {
            cmd.env("NAVE_INPUT", json);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Foreign execution error [{}]: {}", lang, err);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let transmuted_value: Value = serde_json::from_str(stdout.trim()).unwrap_or(Value::Null);
        
        println!("✨ [Transmuter] Successfully transmuted {} result to Nλvescript native type.", lang);
        Ok(transmuted_value)
    }
}

pub fn show_status() -> Result<()> {
    PolyglotBridge::new()?.show_status()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyglot_python_basic() {
        let bridge = PolyglotBridge::new().unwrap();
        let code = "result = 2 + 2";
        let res = bridge.eval_and_transmute("python", code, None).unwrap();
        assert_eq!(res, Value::from(4));
    }

    #[test]
    fn test_polyglot_js_basic() {
        let bridge = PolyglotBridge::new().unwrap();
        let code = "result = 'hello' + ' world';";
        let res = bridge.eval_and_transmute("javascript", code, None).unwrap();
        assert_eq!(res, Value::from("hello world"));
    }

    #[test]
    fn test_polyglot_ruby_basic() {
        let bridge = PolyglotBridge::new().unwrap();
        let code = "result = [1, 2, 3]";
        let res = bridge.eval_and_transmute("ruby", code, None).unwrap();
        let arr = res.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::from(1));
    }

    #[test]
    fn test_polyglot_bash_basic() {
        let bridge = PolyglotBridge::new().unwrap();
        let code = "result='\"success\"'"; // Bash needs to output valid JSON for the transmuter
        let res = bridge.eval_and_transmute("bash", code, None).unwrap();
        assert_eq!(res, Value::from("success"));
    }

    #[test]
    fn test_polyglot_unsupported_language() {
        let bridge = PolyglotBridge::new().unwrap();
        let code = "result = 1";
        let res = bridge.eval_and_transmute("unknown_lang", code, None);
        assert!(res.is_err());
    }
}
