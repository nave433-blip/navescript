// src/nasi.rs
pub trait NasiInterface {
    fn fs_read(&self, path: &str) -> String;
    fn http_fetch(&self, url: &str) -> String;
    fn log(&self, msg: &str);
}

pub struct BrowserNasi;

impl NasiInterface for BrowserNasi {
    fn fs_read(&self, path: &str) -> String {
        format!("(js-bridge: fs_read '{}')", path)
    }

    fn http_fetch(&self, url: &str) -> String {
        format!("(js-bridge: fetch '{}')", url)
    }

    fn log(&self, msg: &str) {
        println!("[NASI Log]: {}", msg);
    }
}

// Utility function to demonstrate usage of NasiInterface
pub fn perform_nasi_ops(nasi: &impl NasiInterface, path: &str, url: &str) -> (String, String) {
    nasi.log("Performing NASI operations");
    (nasi.fs_read(path), nasi.http_fetch(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_nasi() {
        let nasi = BrowserNasi;
        let (fs_res, http_res) = perform_nasi_ops(&nasi, "local.txt", "https://api.example.com");
        assert_eq!(fs_res, "(js-bridge: fs_read 'local.txt')");
        assert_eq!(http_res, "(js-bridge: fetch 'https://api.example.com')");
    }
}
