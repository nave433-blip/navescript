use i2cdev::{I2CDevice, I2CMessage};
use std::fs::File;

pub struct I2CHost {
    device: Box<dyn I2CDevice<File>>,
}

impl I2CHost {
    pub fn new(path: &str, address: u16) -> Result<Self, String> {
        let dev = i2cdev::linux::LinuxI2CDevice::new(path, address)
            .map_err(|e| e.to_string())?;
        Ok(I2CHost { device: Box::new(dev) })
    }

    pub fn read(&mut self, len: u32) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; len as usize];
        self.device.read(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), String> {
        self.device.write(data).map_err(|e| e.to_string())
    }

    pub fn close(&mut self) -> Result<(), String> {
        Ok(())
    }
}
