use crate::polyglot::PolyglotBridge;

pub struct EmbeddingsHost {
    bridge: PolyglotBridge,
}

impl EmbeddingsHost {
    pub fn new() -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(EmbeddingsHost { bridge })
    }

    pub fn vectorize(&self, text: &str) -> Result<Vec<f32>, String> {
        // Example using a Python script via the polyglot bridge
        let code = format!(
            "import numpy as np\n# In a real implementation, load a pre-trained model here\n# Input text: {}\nresult = [0.1, 0.2, 0.3] # Placeholder embedding",
            text
        );
        let res = self.bridge.eval_and_transmute("python", &code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        // Convert JSON array result to Vec<f32>
        let vec: Vec<f32> = serde_json::from_value(res).map_err(|e| e.to_string())?;
        Ok(vec)
    }
}
