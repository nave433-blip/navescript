use crate::polyglot::PolyglotBridge;

pub struct PandasHost {
    bridge: PolyglotBridge,
    json_data: String,
}

impl PandasHost {
    pub fn new(json_data: &str) -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(PandasHost { bridge, json_data: json_data.to_string() })
    }

    pub fn query(&self, q: &str) -> Result<String, String> {
        let code = format!(
            "import pandas as pd\nimport json\ndf = pd.read_json(input_data)\nresult = df.query('{}').to_json()",
            q
        );
        let mut ctx = crate::polyglot::ExecutionContext::default();
        ctx.input_json = Some(self.json_data.clone());
        let res = self.bridge.eval_and_transmute("python", &code, ctx)
            .map_err(|e| e.to_string())?;
        
        Ok(res.to_string())
    }
}
