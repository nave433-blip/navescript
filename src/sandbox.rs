#[derive(Clone)]
pub struct Sandbox {
    pub fs_allowlist: Vec<String>,
    pub net_allowlist: Vec<String>,
    pub max_memory_bytes: usize,
    pub fuel_limit: u64,
    pub timeout_ms: u64,
}

impl Sandbox {
    pub fn strict() -> Self {
        Self {
            fs_allowlist: vec![],
            net_allowlist: vec![],
            max_memory_bytes: 64 * 1024 * 1024,
            fuel_limit: 25_000_000,
            timeout_ms: 5000,
        }
    }

    pub fn permissive() -> Self {
        Self {
            fs_allowlist: vec!["/".to_string()],
            net_allowlist: vec![],
            max_memory_bytes: 512 * 1024 * 1024,
            fuel_limit: 500_000_000,
            timeout_ms: 30000,
        }
    }
}
