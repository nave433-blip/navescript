use anyhow::Result;
use std::process::Command;
use std::env;
use crate::runtime::{NaveRuntime, Value};

pub struct SystemHost;

impl SystemHost {
    pub fn register(rt: &mut NaveRuntime) {
        rt.register_async("system_exec", |args| async move {
            let cmd = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
            let cmd_args: Vec<String> = args.iter().skip(1).map(|v| v.to_string()).collect();
            match Self::exec(cmd, cmd_args) {
                Ok(out) => Value::String(out.trim().to_string()),
                Err(e) => Value::String(format!("Error: {}", e)),
            }
        });

        rt.register_async("system_spawn", |args| async move {
            let cmd = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
            let cmd_args: Vec<String> = args.iter().skip(1).map(|v| v.to_string()).collect();
            match Self::spawn(cmd, cmd_args) {
                Ok(id) => Value::Number(id as f64),
                Err(_) => Value::Nil,
            }
        });

        rt.register_async("system_getenv", |args| async move {
            let name = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
            match Self::get_env(name) {
                Some(v) => Value::String(v),
                None => Value::Nil,
            }
        });

        rt.register_async("system_setenv", |args| async move {
            let name = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
            let value = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
            Self::set_env(name, value);
            Value::Nil
        });

        rt.register_async("system_cwd", |_| async move {
            match Self::cwd() {
                Ok(path) => Value::String(path),
                Err(_) => Value::Nil,
            }
        });

        rt.register_async("system_exit", |args| async move {
            let code = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as i32) } else { None }).unwrap_or(0);
            Self::exit(code);
        });

        rt.register_async("system_info", |_| async move {
            let mut res = std::collections::HashMap::new();
            res.insert("os".to_string(), Value::String(std::env::consts::OS.to_string()));
            res.insert("arch".to_string(), Value::String(std::env::consts::ARCH.to_string()));
            res.insert("family".to_string(), Value::String(std::env::consts::FAMILY.to_string()));
            res.insert("num_cpus".to_string(), Value::Number(num_cpus::get() as f64));
            Value::Object(res)
        });

        rt.register_async("system_env_vars", |_| async move {
            let mut res = std::collections::HashMap::new();
            for (k, v) in std::env::vars() {
                res.insert(k, Value::String(v));
            }
            Value::Object(res)
        });

        rt.register_async("system_args", |_| async move {
            let args: Vec<Value> = std::env::args().map(Value::String).collect();
            Value::Array(args)
        });

        rt.register_async("system_uptime", |_| async move {
            Value::Number(0.0)
        });
    }

    pub fn exec(cmd: &str, args: Vec<String>) -> Result<String> {
        let output = Command::new(cmd)
            .args(args)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            anyhow::bail!("Command failed: {}", String::from_utf8_lossy(&output.stderr))
        }
    }

    pub fn spawn(cmd: &str, args: Vec<String>) -> Result<u32> {
        let child = Command::new(cmd)
            .args(args)
            .spawn()?;
        Ok(child.id())
    }

    pub fn get_env(name: &str) -> Option<String> {
        env::var(name).ok()
    }

    pub fn set_env(name: &str, value: &str) {
        env::set_var(name, value);
    }

    pub fn cwd() -> Result<String> {
        Ok(env::current_dir()?.to_string_lossy().to_string())
    }

    pub fn exit(code: i32) -> ! {
        std::process::exit(code);
    }
}
