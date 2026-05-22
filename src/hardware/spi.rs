use spidev::{Spidev, SpidevOptions, SpiModeFlags, SpidevTransfer};
use std::io::prelude::*;

pub struct SpiHost {
    device: Spidev,
}

impl SpiHost {
    pub fn new(path: &str) -> Result<Self, String> {
        let mut device = Spidev::open(path).map_err(|e| e.to_string())?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(1_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        device.configure(&options).map_err(|e| e.to_string())?;
        Ok(SpiHost { device })
    }

    pub fn transfer(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut rx_buf = vec![0; data.len()];
        {
            let mut transfer = SpidevTransfer::read_write(data, &mut rx_buf);
            self.device.transfer(&mut transfer).map_err(|e| e.to_string())?;
        }
        Ok(rx_buf)
    }

    pub fn close(&mut self) -> Result<(), String> {
        Ok(())
    }
}
