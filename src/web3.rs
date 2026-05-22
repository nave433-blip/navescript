use crate::polyglot::PolyglotBridge;
use serde_json::Value;

pub struct Web3Host {
    bridge: PolyglotBridge,
}

impl Web3Host {
    pub fn new() -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(Web3Host { bridge })
    }

    pub fn get_balance(&self, address: &str) -> Result<String, String> {
        let code = format!(
            "const {address} = '{}';\nconst Web3 = require('web3');\nconst web3 = new Web3('https://mainnet.infura.io/v3/YOUR_INFURA_KEY');\nresult = await web3.eth.getBalance({address});",
            address
        );
        let res = self.bridge.eval_and_transmute("javascript", &code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        Ok(res.as_str().unwrap_or("0").to_string())
    }
}
