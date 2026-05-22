use crate::runtime::{NaveRuntime, Value};
use pcap::{Capture, Device};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Network_capture", |args| async move {
        let interface = args.get(0).and_then(|v| v.as_str()).unwrap_or("eth0");
        let filter = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        
        let device = Device::from(interface);
        let mut cap = Capture::from_device(device).unwrap()
            .promisc(true)
            .snaplen(65535)
            .open().unwrap();

        if !filter.is_empty() {
            cap.filter(filter, true).unwrap();
        }

        let mut packets = Vec::new();
        for _ in 0..10 { // Demo limit
            if let Ok(packet) = cap.next_packet() {
                packets.push(Value::Buffer(packet.data.to_vec()));
            }
        }

        Value::Array(packets)
    });
}
