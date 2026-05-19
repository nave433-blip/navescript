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
