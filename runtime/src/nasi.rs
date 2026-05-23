// navescript/runtime/src/nasi.rs (Expanded with File Handle Management)

use crate::gc::SimpleAllocator;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::Mutex;
use memmap2::MmapMut; // For conceptual mem_grow

// Global map to manage file descriptors that NAS modules 'own'.
// This prevents double-closing or invalid FDs if not carefully managed.
static mut FILE_HANDLES: Option<Mutex<HashMap<u64, OwnedFd>>> = None;
static mut NEXT_HANDLE_ID: u64 = 1;

fn init_file_handles() {
    unsafe {
        if FILE_HANDLES.is_none() {
            FILE_HANDLES = Some(Mutex::new(HashMap::new()));
        }
    }
}

// The NASI syscall handler.
// `memory` is the linear memory slice of the NAS module.
// `allocator` is passed for memory management.
// `mem_mmap` is the actual MmapMut for the module's memory, for mem_grow.
pub fn syscall_handler(id: u32, arg1: u64, arg2: u64, arg3: u64, memory: &mut [u8], allocator: &mut SimpleAllocator, mem_mmap: &mut MmapMut) -> u64 {
    // Ensure file handles are initialized for this thread/process
    init_file_handles();
    let mut handles_guard = unsafe { FILE_HANDLES.as_ref().unwrap().lock().unwrap() };

    match id {
        // Filesystem
        10 => { // fs_open(path_ptr, path_len, flags) -> handle
            let path = get_str_from_mem(memory, arg1, arg2 as usize);
            println!("[NASI] fs_open called for path: {}, flags: {}", path, arg3);
            
            let mut options = OpenOptions::new();
            if arg3 & 0x01 != 0 { options.read(true); } // O_RDONLY
            if arg3 & 0x02 != 0 { options.write(true); } // O_WRONLY
            if arg3 & 0x04 != 0 { options.append(true); } // O_APPEND
            if arg3 & 0x08 != 0 { options.create(true); } // O_CREAT
            
            match options.open(&path) {
                Ok(file) => {
                    let new_handle = unsafe { NEXT_HANDLE_ID };
                    unsafe { NEXT_HANDLE_ID += 1; }
                    handles_guard.insert(new_handle, file.into());
                    new_handle
                },
                Err(e) => {
                    eprintln!("[NASI Error] fs_open: {}: {}", path, e);
                    !0 // Return -1 as u64
                }
            }
        },
        11 => { // fs_read(handle, buf_ptr, buf_len) -> bytes_read
            let handle = arg1;
            let buf_ptr = arg2 as usize;
            let buf_len = arg3 as usize;

            if buf_ptr + buf_len > memory.len() {
                eprintln!("[NASI Error] fs_read: Buffer out of bounds.");
                return 0;
            }

            match handles_guard.get_mut(&handle) {
                Some(owned_fd) => {
                    let mut file = unsafe { File::from_raw_fd(owned_fd.as_raw_fd()) };
                    let bytes_read = match file.read(&mut memory[buf_ptr..buf_ptr + buf_len]) {
                        Ok(n) => n as u64,
                        Err(e) => {
                            eprintln!("[NASI Error] fs_read: {}", e);
                            0
                        }
                    };
                    std::mem::forget(file); // Prevent closing the raw FD
                    bytes_read
                },
                None => {
                    eprintln!("[NASI Error] fs_read: Invalid file handle {}", handle);
                    0
                }
            }
        },
        12 => { // fs_write(handle, buf_ptr, buf_len) -> bytes_written
            let handle = arg1;
            let buf_ptr = arg2 as usize;
            let buf_len = arg3 as usize;

            if buf_ptr + buf_len > memory.len() {
                eprintln!("[NASI Error] fs_write: Buffer out of bounds.");
                return 0;
            }

            if handle == 1 || handle == 2 { // stdout or stderr
                let data = &memory[buf_ptr..buf_ptr + buf_len];
                let s = String::from_utf8_lossy(data);
                if handle == 1 { print!("{}", s); } else { eprint!("{}", s); }
                return buf_len as u64;
            }

            match handles_guard.get_mut(&handle) {
                Some(owned_fd) => {
                    let mut file = unsafe { File::from_raw_fd(owned_fd.as_raw_fd()) };
                    let bytes_written = match file.write(&memory[buf_ptr..buf_ptr + buf_len]) {
                        Ok(n) => n as u64,
                        Err(e) => {
                            eprintln!("[NASI Error] fs_write: {}", e);
                            0
                        }
                    };
                    std::mem::forget(file);
                    bytes_written
                },
                None => {
                    eprintln!("[NASI Error] fs_write: Invalid file handle {}", handle);
                    0
                }
            }
        },
        13 => { // fs_close(handle)
            let handle = arg1;
            println!("[NASI] fs_close called for handle: {}", handle);
            if handles_guard.remove(&handle).is_some() {
                0
            } else {
                eprintln!("[NASI Error] fs_close: Invalid file handle {}", handle);
                !0
            }
        }

        // Memory
        50 => { // mem_grow(pages) -> previous_pages
            println!("[NASI] mem_grow called with pages: {}", arg1);
            // This would involve remapping the MmapMut region.
            // For now, return 0 (no memory grown) and log.
            eprintln!("[NASI Warning] mem_grow not fully implemented.");
            0
        },
        51 => { // gc_alloc(size) -> ptr
            println!("[NASI] gc_alloc called with size: {}", arg1);
            match allocator.alloc(arg1 as usize) {
                Some(ptr) => ptr,
                None => !0, // Return -1 (u64 max) on allocation failure
            }
        }

        // Networking
        100 => { // net_socket_open(family, type) -> handle
            println!("[NASI] net_socket_open called. Family: {}, Type: {}", arg1, arg2);
            !0 // Dummy error
        },
        101 => { // net_socket_connect(handle, addr_ptr, addr_len) -> err
            println!("[NASI] net_socket_connect called. Handle: {}, Addr_ptr: {}, Len: {}", arg1, arg2, arg3);
            !0 // Dummy error
        },
        102 => { // net_socket_write(handle, data_ptr, data_len) -> bytes_written
            println!("[NASI] net_socket_write called. Handle: {}, Data_ptr: {}, Len: {}", arg1, arg2, arg3);
            0 // Dummy
        },

        // Foreign Function Interface (FFI)
        200 => { // ffi_load(lib_path_ptr, lib_path_len) -> lib_handle
            let lib_path = get_str_from_mem(memory, arg1, arg2 as usize);
            println!("[NASI] ffi_load called for library: {}", lib_path);
            !0 // Dummy error
        },
        201 => { // ffi_call(lib_handle, func_name_ptr, func_name_len, args_ptr, args_len) -> result
            println!("[NASI] ffi_call called. Handle: {}, Func_ptr: {}, Args_ptr: {}", arg1, arg2, arg3);
            0 // Dummy
        }

        _ => {
            eprintln!("[NASI] Unknown syscall ID: {}", id);
            0
        }
    }
}

// Helper to get a string from module's linear memory.
fn get_str_from_mem(memory: &[u8], ptr: u64, len: usize) -> String {
    let start = ptr as usize;
    let end = start + len;
    if start < memory.len() && end <= memory.len() {
        String::from_utf8_lossy(&memory[start..end]).to_string()
    } else {
        "<invalid memory address>".to_string()
    }
}
