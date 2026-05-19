// src/ffi.rs
use crate::runtime::NaveRuntime;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn nave_init() -> *mut NaveRuntime {
    let rt = NaveRuntime::new().expect("Failed to init runtime");
    Box::into_raw(Box::new(rt))
}

#[no_mangle]
pub extern "C" fn nave_run(ctx: *mut NaveRuntime, module_path: *const c_char) -> i32 {
    let _rt = unsafe { &*ctx };
    let c_str = unsafe { CStr::from_ptr(module_path) };
    let path = c_str.to_str().unwrap();
    
    println!("Loading module from: {}", path);
    0 // Success
}

#[no_mangle]
pub extern "C" fn nave_free(ctx: *mut NaveRuntime) {
    unsafe { drop(Box::from_raw(ctx)) };
}
