pub mod serial;
pub mod usb;
pub mod hid;
#[cfg(target_os = "linux")]
pub mod i2c;
#[cfg(target_os = "linux")]
pub mod spi;
