use crate::runtime::{NaveRuntime, Value};

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("http_get", |args| async move {
        let url = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        match reqwest::get(url).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => Value::String(text),
                    Err(e) => Value::String(format!("Error reading body: {}", e)),
                }
            },
            Err(e) => Value::String(format!("Error: {}", e)),
        }
    });

    rt.register_async("http_post", |args| async move {
        let url = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let body = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let client = reqwest::Client::new();
        match client.post(url).body(body.to_string()).send().await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => Value::String(text),
                    Err(e) => Value::String(format!("Error reading body: {}", e)),
                }
            },
            Err(e) => Value::String(format!("Error: {}", e)),
        }
    });
}
