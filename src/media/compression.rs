use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::prelude::*;

pub struct CompressionHost;

impl CompressionHost {
    pub fn new() -> Self {
        CompressionHost {}
    }

    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).map_err(|e| e.to_string())?;
        encoder.finish().map_err(|e| e.to_string())
    }

    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = GzDecoder::new(data);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }
}
