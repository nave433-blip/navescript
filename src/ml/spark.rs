use crate::polyglot::PolyglotBridge;

pub struct SparkHost {
    bridge: PolyglotBridge,
    script: String,
}

impl SparkHost {
    pub fn new(script: &str) -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(SparkHost { bridge, script: script.to_string() })
    }

    pub fn submit(&self) -> Result<String, String> {
        // Orchestrate Spark via spark-submit
        let code = format!("import os\nos.system('spark-submit --master local {}')", self.script);
        let res = self.bridge.eval_and_transmute("python", &code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        Ok(res.to_string())
    }
}
