use rusb::{Device, DeviceHandle, GlobalContext};
use std::time::Duration;

pub struct UsbHost {
    handle: DeviceHandle<GlobalContext>,
}

impl UsbHost {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Self, String> {
        let handle = rusb::open_device_with_vid_pid(vendor_id, product_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        Ok(UsbHost { handle })
    }

    pub fn transfer(&mut self, endpoint: u8, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; 64]; // Simplified buffer size
        self.handle.write_interrupt(endpoint, data, Duration::from_millis(1000))
            .map_err(|e| e.to_string())?;
        self.handle.read_interrupt(endpoint, &mut buffer, Duration::from_millis(1000))
            .map_err(|e| e.to_string())?;
        Ok(buffer)
    }

    pub fn close(&mut self) -> Result<(), String> {
        // Handle closing
        Ok(())
    }
}
