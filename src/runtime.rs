use wasmtime::{Config, Engine, Module, Store, Trap, OptLevel, Linker, Caller};
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use crate::ir::StepIr;
use crate::nasi::HostNasi;
use crate::sandbox::Sandbox;
use crate::ns_compiler::TargetCompiler;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    BigInt(i128),
    Rational(i64, i64),
    Complex(f64, f64),
    Array(Vec<Value>),
    Buffer(Vec<u8>),
    Object(std::collections::HashMap<String, Value>),
    Map(std::collections::HashMap<Value, Value>),
    Set(std::collections::HashSet<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::BigInt(n) => write!(f, "{}n", n),
            Value::Rational(n, d) => write!(f, "{}r{}", n, d),
            Value::Complex(r, i) => write!(f, "{}+{}i", r, i),
            Value::Array(a) => write!(f, "[Array({})]", a.len()),
            Value::Buffer(b) => write!(f, "[Buffer({})]", b.len()),
            Value::Object(_) => write!(f, "[Object]"),
            Value::Map(m) => write!(f, "[Map({})]", m.len()),
            Value::Set(s) => write!(f, "[Set({})]", s.len()),
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => 0.hash(state),
            Value::Number(n) => n.to_bits().hash(state),
            Value::String(s) => s.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::BigInt(n) => n.hash(state),
            Value::Rational(n, d) => { n.hash(state); d.hash(state); }
            Value::Complex(r, i) => { r.to_bits().hash(state); i.to_bits().hash(state); }
            Value::Array(a) => a.hash(state),
            Value::Buffer(b) => b.hash(state),
            Value::Object(o) => {
                let mut keys: Vec<_> = o.keys().collect();
                keys.sort();
                for k in keys {
                    k.hash(state);
                    o.get(k).unwrap().hash(state);
                }
            }
            Value::Map(m) => {
                for (k, v) in m {
                    k.hash(state);
                    v.hash(state);
                }
            }
            Value::Set(s) => {
                for v in s {
                    v.hash(state);
                }
            }
        }
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self { Some(s) } else { None }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::BigInt(n) => *n != 0,
            Value::Rational(n, _) => *n != 0,
            Value::Array(a) => !a.is_empty(),
            Value::Buffer(b) => !b.is_empty(),
            Value::Object(o) => !o.is_empty(),
            _ => true,
        }
    }
}

pub struct NaveRuntime {
    pub nasi: HostNasi,
    pub functions: HashMap<String, (Vec<String>, Vec<StepIr>)>,
    pub async_funcs: HashMap<String, Box<dyn Fn(Vec<Value>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Value> + Send>> + Send + Sync>>,
    _variable_memory: HashMap<String, serde_json::Value>,
    pub sandbox: Sandbox,
}

impl NaveRuntime {
    pub fn new(sandbox: Option<Sandbox>) -> Result<Self> {
        let sandbox = sandbox.unwrap_or_else(Sandbox::permissive);
        let mut rt = Self {
            nasi: HostNasi::new(),
            functions: HashMap::new(),
            async_funcs: HashMap::new(),
            _variable_memory: HashMap::new(),
            sandbox,
        };
        crate::stdlib::register(&mut rt);
        crate::polyglot::PolyglotBridge::register(&mut rt);
        crate::security::register(&mut rt);
        Ok(rt)
    }

    pub fn register_async<F, Fut>(&mut self, name: &str, f: F)
    where
        F: Fn(Vec<Value>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Value> + Send + 'static,
    {
        self.async_funcs.insert(name.to_string(), Box::new(move |args| Box::pin(f(args))));
    }

    pub async fn run_wasm_target(&mut self, ir: &[StepIr]) -> Result<serde_json::Value> {
        let mut config = Config::new();
        config.wasm_simd(true);
        config.wasm_bulk_memory(true);
        config.wasm_multi_value(true);
        config.wasm_reference_types(true);
        config.cranelift_opt_level(OptLevel::SpeedAndSize);
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);

        let engine = Engine::new(&config)
            .context("Failed to create SIMD-enabled Wasmtime Engine")?;

        let mut store: Store<()> = Store::new(&engine, ());
        store.set_fuel(self.sandbox.fuel_limit)?;
        store.epoch_deadline_callback(|_store| {
            Ok(wasmtime::UpdateDeadline::Yield(0))
        });

        let mut linker = Linker::new(&engine);
        self.register_host_functions(&mut linker)?;

        let wat = TargetCompiler::ir_to_wat(ir);
        let module = Module::new(&engine, &wat)
            .context("Failed to compile IR to WASM module")?;

        let instance = linker.instantiate(&mut store, &module)
            .context("Failed to instantiate WASM module")?;

        let execution = async {
            if let Ok(func) = instance.get_typed_func::<(), ()>(&mut store, "main") {
                func.call_async(&mut store, ()).await.map_err(map_wasmtime_error)
            } else {
                Ok(())
            }
        };

        match timeout(Duration::from_millis(self.sandbox.timeout_ms), execution).await {
            Ok(Ok(_)) => Ok(serde_json::json!("✅ WASM (SIMD + Cranelift) execution successful")),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow!("⏰ Execution timed out.")),
        }
    }

    fn register_host_functions(&self, linker: &mut Linker<()>) -> Result<()> {
        linker.func_wrap("env", "nave_print", |mut caller: Caller<()>, ptr: u32, len: u32| {
            let mem = caller.get_export("memory")
                .and_then(|e| e.into_memory())
                .ok_or_else(|| anyhow::anyhow!("Memory out of bounds"))?;
            let data = mem.data(&caller);
            let s = String::from_utf8_lossy(&data[ptr as usize..(ptr + len) as usize]);
            println!("[Nλve Host] {}", s);
            Ok(())
        })?;
        Ok(())
    }
}

fn map_wasmtime_error(err: wasmtime::Error) -> anyhow::Error {
    if let Some(trap) = err.downcast_ref::<Trap>() {
        match *trap {
            Trap::OutOfFuel => anyhow!("⛽ Fuel exhausted"),
            Trap::StackOverflow => anyhow!("💥 Stack overflow"),
            Trap::MemoryOutOfBounds => anyhow!("📛 Memory violation"),
            _ => anyhow!("💥 WASM Trap: {:?}", trap),
        }
    } else {
        anyhow!("❌ WASM Error: {}", err)
    }
}
