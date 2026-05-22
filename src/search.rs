use crate::polyglot::PolyglotBridge;

pub struct MeilisearchHost {
    bridge: PolyglotBridge,
    url: String,
    api_key: String,
}

impl MeilisearchHost {
    pub fn new(url: &str, api_key: &str) -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(MeilisearchHost { 
            bridge, 
            url: url.to_string(), 
            api_key: api_key.to_string() 
        })
    }

    pub fn search(&self, index: &str, query: &str) -> Result<Vec<String>, String> {
        let code = format!(
            "import meilisearch\nclient = meilisearch.Client('{}', '{}')\nindex = client.index('{}')\nresult = [h['id'] for h in index.search('{}')['hits']]",
            self.url, self.api_key, index, query
        );
        let res = self.bridge.eval_and_transmute("python", &code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        let vec: Vec<String> = serde_json::from_value(res).map_err(|e| e.to_string())?;
        Ok(vec)
    }
}
