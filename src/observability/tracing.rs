use opentelemetry::trace::{Tracer as _, Span as TraceSpan, TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use opentelemetry_sdk::trace::Tracer;

pub struct TracingHost {
    spans: Arc<Mutex<HashMap<u64, opentelemetry_sdk::trace::Span>>>,
    tracer: Tracer,
}

impl TracingHost {
    pub fn new() -> Result<Self, String> {
        let provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_endpoint("http://localhost:4317"))
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .map_err(|e| e.to_string())?;

        let tracer = provider.tracer("navescript");

        Ok(TracingHost {
            spans: Arc::new(Mutex::new(HashMap::new())),
            tracer,
        })
    }

    pub fn start_span(&self, name: String) -> u64 {
        let span = self.tracer.start(name);
        let id = rand::random();
        self.spans.lock().unwrap().insert(id, span);
        id
    }

    pub fn end_span(&self, id: u64) {
        if let Some(mut span) = self.spans.lock().unwrap().remove(&id) {
            span.end();
        }
    }

    pub fn record_event(&self, id: u64, name: String) {
        let mut spans = self.spans.lock().unwrap();
        if let Some(span) = spans.get_mut(&id) {
            span.add_event(name, vec![]);
        }
    }
}
