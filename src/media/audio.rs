use hound::{WavReader, WavWriter, WavSpec};
use std::io::Cursor;

pub struct AudioHost {
    samples: Vec<i32>,
    spec: WavSpec,
}

impl AudioHost {
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let cursor = Cursor::new(data);
        let mut reader = WavReader::new(cursor).map_err(|e| e.to_string())?;
        let spec = reader.spec();
        let samples: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
        Ok(AudioHost { samples, spec })
    }

    pub fn gain(&mut self, factor: f32) -> Result<(), String> {
        self.samples = self.samples.iter()
            .map(|&s| (s as f32 * factor) as i32)
            .collect();
        Ok(())
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut buffer = Cursor::new(Vec::new());
        let mut writer = WavWriter::new(&mut buffer, self.spec).unwrap();
        for &s in &self.samples {
            writer.write_sample(s).unwrap();
        }
        writer.finalize().unwrap();
        buffer.into_inner()
    }
}
