// src/hardware/android.rs
use super::{HardwareAbstraction, CpuFeatures, StorageInfo, Vector3, Location};

pub struct AndroidHardware;

impl HardwareAbstraction for AndroidHardware {
    fn cpu_features() -> CpuFeatures {
        CpuFeatures { neon: true, sve: true, aes: true, sha: true, fp16: true }
    }
    fn cpu_temperature() -> Option<f32> { Some(42.5) }
    fn cpu_frequency() -> u64 { 2400000000 }
    fn core_count() -> usize { 8 }
    fn total_ram() -> u64 { 12 * 1024 * 1024 * 1024 }
    fn available_ram() -> u64 { 4 * 1024 * 1024 * 1024 }
    fn storage_info() -> StorageInfo { StorageInfo { total: 256 * 1024 * 1024 * 1024, free: 128 * 1024 * 1024 * 1024 } }
    fn battery_level() -> f32 { 0.65 }
    fn battery_charging() -> bool { true }
    fn accelerometer() -> Option<Vector3> { Some(Vector3 { x: 0.1, y: 0.1, z: 9.7 }) }
    fn gyroscope() -> Option<Vector3> { Some(Vector3 { x: 0.0, y: 0.0, z: 0.0 }) }
    fn magnetometer() -> Option<Vector3> { Some(Vector3 { x: 10.0, y: 5.0, z: -20.0 }) }
    fn light_sensor() -> Option<f32> { Some(500.0) }
    fn proximity_sensor() -> Option<f32> { Some(0.0) }
    fn gps_location() -> Option<Location> { Some(Location { latitude: 34.0522, longitude: -118.2437 }) }
}
