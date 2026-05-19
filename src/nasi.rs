// NASI Host Runtime: Capability-based system caller
pub struct NasiHost {
    allowed_caps: Vec<String>,
}

impl NasiHost {
    pub fn new(caps: Vec<String>) -> Self {
        Self { allowed_caps: caps }
    }

    pub fn execute_syscall(&self, id: u32, args: &[u8]) -> Result<u32, String> {
        let cap = match id {
            1 => "fs:read",
            2 => "fs:write",
            3 => "net:connect",
            _ => "unknown",
        };

        if !self.allowed_caps.contains(&cap.to_string()) {
            return Err(format!("Permission Denied: Missing capability '{}'", cap));
        }

        println!("⚡ Executing NASI syscall: {} (id: {})", cap, id);
        Ok(0)
    }
}
