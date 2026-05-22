use hidapi::{HidApi, HidDevice};

pub struct HidHost {
    device: HidDevice,
}

impl HidHost {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Self, String> {
        let api = HidApi::new().map_err(|e| e.to_string())?;
        let device = api.open(vendor_id, product_id)
            .map_err(|e| e.to_string())?;
        
        Ok(HidHost { device })
    }

    pub fn read(&self, len: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; len];
        let bytes_read = self.device.read(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer[..bytes_read].to_vec())
    }

    pub fn write(&self, data: &[u8]) -> Result<(), String> {
        self.device.write(data).map_err(|e| e.to_string())?;
        Ok(())
    }
}
