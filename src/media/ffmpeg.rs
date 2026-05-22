use crate::polyglot::PolyglotBridge;

pub struct FfmpegHost {
    bridge: PolyglotBridge,
    command: String,
}

impl FfmpegHost {
    pub fn new(command: &str) -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(FfmpegHost { bridge, command: command.to_string() })
    }

    pub fn run(&self) -> Result<String, String> {
        let code = format!("result=$(ffmpeg {} 2>&1)", self.command);
        let res = self.bridge.eval_and_transmute("bash", &code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        Ok(res.to_string())
    }
}
