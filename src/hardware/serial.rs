use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct SerialHost {
    stream: SerialStream,
}

impl SerialHost {
    pub async fn new(port: &str, baud_rate: u32) -> Result<Self, String> {
        let stream = tokio_serial::new(port, baud_rate)
            .open_native_async()
            .map_err(|e| e.to_string())?;
        Ok(SerialHost { stream })
    }

    pub async fn read(&mut self, len: u32) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; len as usize];
        self.stream.read_exact(&mut buffer).await.map_err(|e| e.to_string())?;
        Ok(buffer)
    }

    pub async fn write(&mut self, data: Vec<u8>) -> Result<u64, String> {
        let written = self.stream.write(&data).await.map_err(|e| e.to_string())?;
        Ok(written as u64)
    }

    pub async fn close(&mut self) -> Result<(), String> {
        // tokio-serial streams close automatically when dropped.
        Ok(())
    }
}
