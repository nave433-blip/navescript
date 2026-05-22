// src/hardware/ios.rs
use super::{HardwareAbstraction, CpuFeatures, StorageInfo, Vector3, Location};

pub struct IosHardware;

impl HardwareAbstraction for IosHardware {
    fn cpu_features() -> CpuFeatures {
        CpuFeatures { neon: true, sve: false, aes: true, sha: true, fp16: true }
    }
    fn cpu_temperature() -> Option<f32> { None }
    fn cpu_frequency() -> u64 { 0 }
    fn core_count() -> usize { 6 }
    fn total_ram() -> u64 { 8 * 1024 * 1024 * 1024 }
    fn available_ram() -> u64 { 2 * 1024 * 1024 * 1024 }
    fn storage_info() -> StorageInfo { StorageInfo { total: 128 * 1024 * 1024 * 1024, free: 32 * 1024 * 1024 * 1024 } }
    fn battery_level() -> f32 { 0.85 }
    fn battery_charging() -> bool { false }
    fn accelerometer() -> Option<Vector3> { Some(Vector3 { x: 0.0, y: 0.0, z: 9.8 }) }
    fn gyroscope() -> Option<Vector3> { None }
    fn magnetometer() -> Option<Vector3> { None }
    fn light_sensor() -> Option<f32> { Some(100.0) }
    fn proximity_sensor() -> Option<f32> { Some(1.0) }
    fn gps_location() -> Option<Location> { Some(Location { latitude: 37.7749, longitude: -122.4194 }) }
}
