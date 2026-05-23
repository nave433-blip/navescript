// navescript/runtime/src/nasi.rs (Expanded)

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

// The NASI syscall handler.
// `memory` is the linear memory slice of the NAS module.
// `gc_allocator` is passed for memory management.
pub fn syscall_handler(id: u32, arg1: u64, arg2: u64, arg3: u64, memory: &mut [u8]) -> u64 {
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
                Ok(file) => file.as_raw_fd() as u64, // Return file descriptor as handle
                Err(e) => {
                    eprintln!("[NASI Error] fs_open: {}: {}", path, e);
                    !0 // Return -1 as u64
                }
            }
        },
        11 => { // fs_read(handle, buf_ptr, buf_len) -> bytes_read
            let fd = arg1 as i32;
            let buf_ptr = arg2 as usize;
            let buf_len = arg3 as usize;

            if buf_ptr + buf_len > memory.len() {
                eprintln!("[NASI Error] fs_read: Buffer out of bounds.");
                return 0;
            }

            // SAFETY: We are creating an owned file descriptor from a raw one.
            // This is inherently unsafe if the fd is invalid or not owned by us.
            // In a real system, we'd manage a FileHandle table.
            let mut file = unsafe { File::from_raw_fd(fd) };
            let bytes_read = match file.read(&mut memory[buf_ptr..buf_ptr + buf_len]) {
                Ok(n) => n as u64,
                Err(e) => {
                    eprintln!("[NASI Error] fs_read: {}", e);
                    0
                }
            };
            // SAFETY: Prevent file from being closed on drop if we don't own it.
            std::mem::forget(file);
            bytes_read
        },
        12 => { // fs_write(handle, buf_ptr, buf_len) -> bytes_written
            let fd = arg1 as i32;
            let buf_ptr = arg2 as usize;
            let buf_len = arg3 as usize;

            if buf_ptr + buf_len > memory.len() {
                eprintln!("[NASI Error] fs_write: Buffer out of bounds.");
                return 0;
            }

            // Handle stdout/stderr specially
            if fd == 1 || fd == 2 { // stdout or stderr
                let data = &memory[buf_ptr..buf_ptr + buf_len];
                let s = String::from_utf8_lossy(data);
                if fd == 1 { print!("{}", s); } else { eprint!("{}", s); }
                return buf_len as u64;
            }

            // SAFETY: Similar to fs_read, this is unsafe.
            let mut file = unsafe { File::from_raw_fd(fd) };
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
        13 => { // fs_close(handle)
            let fd = arg1 as i32;
            println!("[NASI] fs_close called for handle: {}", fd);
            // SAFETY: We are taking ownership of the fd.
            // This assumes the NAS module gives up ownership.
            let _file = unsafe { File::from_raw_fd(fd) };
            0
        }

        // Memory
        50 => { // mem_grow(pages) -> previous_pages
            println!("[NASI] mem_grow called with pages: {}", arg1);
            // TODO: Implement actual memory growth.
            0
        },
        51 => { // gc_alloc(size) -> ptr
            println!("[NASI] gc_alloc called with size: {}", arg1);
            // This needs to be integrated with the GC system.
            // For now, return a dummy address within the linear memory.
            1024 // Return a dummy pointer
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
