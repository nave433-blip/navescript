pub mod console;
pub mod fs;
pub mod path;
pub mod http;
pub mod crypto;
pub mod system;
pub mod intl;
pub mod timer;
pub mod error;
pub mod bootstrap;
pub mod hw;
pub mod db;

pub fn register(rt: &mut crate::runtime::NaveRuntime) {
    console::register(rt);
    fs::register(rt);
    path::register(rt);
    http::register(rt);
    crypto::register(rt);
    system::SystemHost::register(rt);
    intl::register(rt);
    timer::register(rt);
    error::register(rt);
    hw::HardwareHost::register(rt);
    db::DatabaseHost::register(rt);
}
