use prometheus::{Encoder, TextEncoder, Registry, Counter, Gauge, opts};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MetricsHost {
    registry: Registry,
    counters: Arc<Mutex<HashMap<String, Counter>>>,
    gauges: Arc<Mutex<HashMap<String, Gauge>>>,
}

impl MetricsHost {
    pub fn new() -> Self {
        MetricsHost {
            registry: Registry::new(),
            counters: Arc::new(Mutex::new(HashMap::new())),
            gauges: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc_counter(&self, name: &str, help: &str) {
        let mut counters = self.counters.lock().unwrap();
        let counter = counters.entry(name.to_string()).or_insert_with(|| {
            let c = Counter::new(name, help).unwrap();
            self.registry.register(Box::new(c.clone())).unwrap();
            c
        });
        counter.inc();
    }

    pub fn observe_gauge(&self, name: &str, help: &str, value: f64) {
        let mut gauges = self.gauges.lock().unwrap();
        let gauge = gauges.entry(name.to_string()).or_insert_with(|| {
            let g = Gauge::new(name, help).unwrap();
            self.registry.register(Box::new(g.clone())).unwrap();
            g
        });
        gauge.set(value);
    }
}
