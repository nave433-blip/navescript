// Nvsr: NaveScript-Rust Bridge
use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "nave")]
extern "C" {
    fn nave_init() -> *mut std::ffi::c_void;
    fn nave_run(ctx: *mut std::ffi::c_void, path: *const c_char) -> i32;
    fn nave_free(ctx: *mut std::ffi::c_void);
}

pub struct Nvsr {
    ctx: *mut std::ffi::c_void,
}

impl Nvsr {
    pub fn new() -> Self {
        unsafe { Self { ctx: nave_init() } }
    }
    
    pub fn run(&self, path: &str) -> i32 {
        let c_path = std::ffi::CString::new(path).unwrap();
        unsafe { nave_run(self.ctx, c_path.as_ptr()) }
    }
}
