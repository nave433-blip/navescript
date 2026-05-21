// src/nasi.rs
use std::fs;
use std::io::{self};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum NasiError {
    #[error("Success")]
    Success = 0,
    #[error("Argument list too long")]
    TooBig = 1,
    #[error("Permission denied")]
    Access = 2,
    #[error("Address already in use")]
    AddrInUse = 3,
    #[error("Address not available")]
    AddrNotAvailable = 4,
    #[error("Resource unavailable, try again")]
    Again = 5,
    #[error("Connection already in progress")]
    Already = 6,
    #[error("Bad file descriptor")]
    Badf = 7,
    #[error("Device or resource busy")]
    Busy = 8,
    #[error("Connection aborted")]
    ConnectionAborted = 9,
    #[error("Connection refused")]
    ConnectionRefused = 10,
    #[error("Connection reset")]
    ConnectionReset = 11,
    #[error("Resource deadlock would occur")]
    Deadlock = 12,
    #[error("File exists")]
    Exist = 13,
    #[error("Invalid argument")]
    Inval = 14,
    #[error("I/O error")]
    Io = 15,
    #[error("Is a directory")]
    Isdir = 16,
    #[error("Too many levels of symbolic links")]
    Loop = 17,
    #[error("No such file or directory")]
    Noent = 18,
    #[error("Exec format error")]
    Noexec = 19,
    #[error("No locks available")]
    Nolck = 20,
    #[error("Not enough space")]
    Nomem = 21,
    #[error("No space left on device")]
    Nospc = 22,
    #[error("Function not implemented")]
    Nosys = 23,
    #[error("Not a directory")]
    Notdir = 24,
    #[error("Directory not empty")]
    Notempty = 25,
    #[error("State not recoverable")]
    Notrecoverable = 26,
    #[error("Operation not permitted")]
    Perm = 27,
    #[error("Broken pipe")]
    Pipe = 28,
    #[error("Disk quota exceeded")]
    Quot = 29,
    #[error("Result too large")]
    Range = 30,
    #[error("Read-only file system")]
    Rofs = 31,
    #[error("Illegal seek")]
    Spipe = 32,
    #[error("Connection timed out")]
    Timedout = 33,
    #[error("Operation would block")]
    Wouldblock = 34,
    #[error("Cross-device link")]
    Xdev = 35,
    #[error("Operation not supported")]
    NotSup = 36,
    #[error("Operation canceled")]
    Canceled = 37,
    #[error("Retry")]
    Retry = 38,
    #[error("Value too large to be stored in target type")]
    Overflow = 39,
}

impl NasiError {
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
}

pub type NasiResult<T> = Result<T, NasiError>;

impl From<io::Error> for NasiError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => NasiError::Noent,
            io::ErrorKind::PermissionDenied => NasiError::Perm,
            io::ErrorKind::AlreadyExists => NasiError::Exist,
            io::ErrorKind::InvalidInput => NasiError::Inval,
            io::ErrorKind::WouldBlock => NasiError::Wouldblock,
            io::ErrorKind::BrokenPipe => NasiError::Pipe,
            io::ErrorKind::TimedOut => NasiError::Timedout,
            _ => NasiError::Io,
        }
    }
}

impl From<wasmtime::Error> for NasiError {
    fn from(_err: wasmtime::Error) -> Self {
        NasiError::Io
    }
}

pub trait NasiInterface {
    fn fs_read(&self, path: &str) -> NasiResult<String>;
    fn fs_write(&self, path: &str, data: &str) -> NasiResult<()>;
    fn fs_remove_file(&self, path: &str) -> NasiResult<()>;
    fn fs_exists(&self, path: &str) -> bool;
    fn http_get(&self, url: &str) -> NasiResult<String>;
    fn http_post(&self, url: &str, body: &str) -> NasiResult<String>;
    fn stdout_write(&self, msg: &str) -> NasiResult<()>;
    fn stderr_write(&self, msg: &str) -> NasiResult<()>;
    fn stdin_read(&self) -> NasiResult<String>;
    fn proc_exit(&self, status: u32) -> !;
    fn wall_clock_now(&self) -> u64;
    fn monotonic_clock_now(&self) -> u64;
    fn random_get_u32(&self) -> u32;
    fn random_get_bytes(&self, len: u32) -> Vec<u8>;
}

pub struct HostNasi;

impl NasiInterface for HostNasi {
    fn fs_read(&self, path: &str) -> NasiResult<String> {
        fs::read_to_string(path).map_err(NasiError::from)
    }

    fn fs_write(&self, path: &str, data: &str) -> NasiResult<()> {
        fs::write(path, data).map_err(NasiError::from)
    }

    fn fs_remove_file(&self, path: &str) -> NasiResult<()> {
        fs::remove_file(path).map_err(NasiError::from)
    }

    fn fs_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn http_get(&self, url: &str) -> NasiResult<String> {
        let output = std::process::Command::new("curl").arg("-s").arg("-f").arg(url).output().map_err(NasiError::from)?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(NasiError::Io)
        }
    }

    fn http_post(&self, url: &str, body: &str) -> NasiResult<String> {
        let output = std::process::Command::new("curl").arg("-s").arg("-f").arg("-X").arg("POST").arg("-d").arg(body).arg(url).output().map_err(NasiError::from)?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(NasiError::Io)
        }
    }

    fn stdout_write(&self, msg: &str) -> NasiResult<()> {
        use std::io::Write;
        print!("{}", msg);
        io::stdout().flush().map_err(NasiError::from)
    }

    fn stderr_write(&self, msg: &str) -> NasiResult<()> {
        use std::io::Write;
        eprint!("{}", msg);
        io::stderr().flush().map_err(NasiError::from)
    }

    fn stdin_read(&self) -> NasiResult<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(NasiError::from)?;
        Ok(input.trim().to_string())
    }

    fn proc_exit(&self, status: u32) -> ! {
        std::process::exit(status as i32);
    }

    fn wall_clock_now(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    fn monotonic_clock_now(&self) -> u64 {
        std::time::Instant::now().elapsed().as_secs()
    }

    fn random_get_u32(&self) -> u32 {
        use rand::Rng;
        rand::thread_rng().gen()
    }

    fn random_get_bytes(&self, len: u32) -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..len).map(|_| rng.gen()).collect()
    }
}

pub struct BrowserNasi;

impl NasiInterface for BrowserNasi {
    fn fs_read(&self, _path: &str) -> NasiResult<String> { Err(NasiError::NotSup) }
    fn fs_write(&self, _path: &str, _data: &str) -> NasiResult<()> { Err(NasiError::NotSup) }
    fn fs_remove_file(&self, _path: &str) -> NasiResult<()> { Err(NasiError::NotSup) }
    fn fs_exists(&self, _path: &str) -> bool { false }
    fn http_get(&self, _url: &str) -> NasiResult<String> { Err(NasiError::NotSup) }
    fn http_post(&self, _url: &str, _body: &str) -> NasiResult<String> { Err(NasiError::NotSup) }
    fn stdout_write(&self, msg: &str) -> NasiResult<()> { println!("[Browser]: {}", msg); Ok(()) }
    fn stderr_write(&self, msg: &str) -> NasiResult<()> { eprintln!("[Browser]: {}", msg); Ok(()) }
    fn stdin_read(&self, ) -> NasiResult<String> { Err(NasiError::NotSup) }
    fn proc_exit(&self, _status: u32) -> ! { panic!("proc_exit"); }
    fn wall_clock_now(&self) -> u64 { 0 }
    fn monotonic_clock_now(&self) -> u64 { 0 }
    fn random_get_u32(&self) -> u32 { 42 }
    fn random_get_bytes(&self, len: u32) -> Vec<u8> { vec![0; len as usize] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_nasi_clocks() {
        let nasi = HostNasi;
        let now = nasi.wall_clock_now();
        assert!(now > 0);
    }
}
