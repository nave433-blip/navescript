// src/stdlib/hw.rs
use crate::runtime::{NaveRuntime, Value};
use crate::hardware::{serial, usb};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub struct HardwareHost {
    serial_ports: Arc<Mutex<HashMap<String, serial::SerialHost>>>,
    usb_devices: Arc<Mutex<HashMap<String, usb::UsbHost>>>,
}

impl HardwareHost {
    pub fn register(rt: &mut NaveRuntime) {
        let host = Arc::new(Self {
            serial_ports: Arc::new(Mutex::new(HashMap::new())),
            usb_devices: Arc::new(Mutex::new(HashMap::new())),
        });

        let h = host.clone();
        rt.register_async("hw_serial_open", move |args| {
            let h = h.clone();
            async move {
                let port = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let baud = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None }).unwrap_or(9600);
                
                match serial::SerialHost::new(port, baud).await {
                    Ok(s) => {
                        let id = format!("serial_{}", port);
                        h.serial_ports.lock().await.insert(id.clone(), s);
                        Value::String(id)
                    }
                    Err(e) => Value::String(format!("Error: {}", e)),
                }
            }
        });

        let h = host.clone();
        rt.register_async("hw_serial_read", move |args| {
            let h = h.clone();
            async move {
                let id = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let len = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None }).unwrap_or(1);
                
                let mut ports = h.serial_ports.lock().await;
                if let Some(s) = ports.get_mut(id) {
                    match s.read(len).await {
                        Ok(data) => Value::Array(data.into_iter().map(|b| Value::Number(b as f64)).collect()),
                        Err(e) => Value::String(format!("Error: {}", e)),
                    }
                } else {
                    Value::String("Error: Port not open".to_string())
                }
            }
        });

        let h = host.clone();
        rt.register_async("hw_serial_write", move |args| {
            let h = h.clone();
            async move {
                let id = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let data = args.get(1).and_then(|v| if let Value::Array(a) = v { 
                    Some(a.iter().filter_map(|v| if let Value::Number(n) = v { Some(*n as u8) } else { None }).collect::<Vec<u8>>())
                } else { None }).unwrap_or_default();
                
                let mut ports = h.serial_ports.lock().await;
                if let Some(s) = ports.get_mut(id) {
                    match s.write(data).await {
                        Ok(n) => Value::Number(n as f64),
                        Err(e) => Value::String(format!("Error: {}", e)),
                    }
                } else {
                    Value::String("Error: Port not open".to_string())
                }
            }
        });

        let h = host.clone();
        rt.register_async("hw_usb_open", move |args| {
            let h = h.clone();
            async move {
                let vid = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u16) } else { None }).unwrap_or(0);
                let pid = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u16) } else { None }).unwrap_or(0);
                
                match usb::UsbHost::new(vid, pid) {
                    Ok(u) => {
                        let id = format!("usb_{}_{}", vid, pid);
                        h.usb_devices.lock().await.insert(id.clone(), u);
                        Value::String(id)
                    }
                    Err(e) => Value::String(format!("Error: {}", e)),
                }
            }
        });
    }
}
