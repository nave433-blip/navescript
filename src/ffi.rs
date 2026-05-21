// src/ffi.rs
use crate::runtime::NaveRuntime;
use crate::parser;
use crate::ir;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::fs;

#[no_mangle]
pub extern "C" fn nave_init() -> *mut NaveRuntime {
    let rt = NaveRuntime::new().expect("Failed to init runtime");
    Box::into_raw(Box::new(rt))
}

#[no_mangle]
pub extern "C" fn nave_run(ctx: *mut NaveRuntime, module_path: *const c_char) -> i32 {
    let rt = unsafe { &mut *ctx };
    let c_str = unsafe { CStr::from_ptr(module_path) };
    let path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    
    println!("🚀 [FFI] Loading module from: {}", path);
    
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let prog = match parser::parse(&source) {
        Ok(p) => p,
        Err(_) => return -3,
    };

    let ir = ir::NSIr::from_program(&prog);

    match tokio::runtime::Runtime::new().unwrap().block_on(rt.interpret_ir(&ir)) {
        Ok(_) => 0,
        Err(_) => -4,
    }
}

#[no_mangle]
pub extern "C" fn nave_free(ctx: *mut NaveRuntime) {
    if !ctx.is_null() {
        unsafe { drop(Box::from_raw(ctx)) };
    }
}
