// src/stdlib/db.rs
use crate::runtime::{NaveRuntime, Value};
use crate::drivers::db::DbHost;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub struct DatabaseHost {
    connections: Arc<Mutex<HashMap<String, DbHost>>>,
}

impl DatabaseHost {
    pub fn register(rt: &mut NaveRuntime) {
        let host = Arc::new(Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        });

        let h = host.clone();
        rt.register_async("db_open", move |args| {
            let h = h.clone();
            async move {
                let url = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let db_type = args.get(1).and_then(|v| v.as_str()).unwrap_or("sqlite");
                
                match DbHost::new(url, db_type).await {
                    Ok(db) => {
                        let id = format!("db_{}_{}", db_type, url);
                        h.connections.lock().await.insert(id.clone(), db);
                        Value::String(id)
                    }
                    Err(e) => Value::String(format!("Error: {}", e)),
                }
            }
        });

        let h = host.clone();
        rt.register_async("db_execute", move |args| {
            let h = h.clone();
            async move {
                let id = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let query = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
                
                let conns = h.connections.lock().await;
                if let Some(db) = conns.get(id) {
                    match db.execute(query).await {
                        Ok(n) => Value::Number(n as f64),
                        Err(e) => Value::String(format!("Error: {}", e)),
                    }
                } else {
                    Value::String("Error: DB not open".to_string())
                }
            }
        });

        let h = host.clone();
        rt.register_async("db_query", move |args| {
            let h = h.clone();
            async move {
                let id = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let query = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
                
                let conns = h.connections.lock().await;
                if let Some(db) = conns.get(id) {
                    match db.query(query).await {
                        Ok(rows) => {
                            let results = rows.into_iter()
                                .map(|row| Value::Array(row.into_iter().map(Value::String).collect()))
                                .collect();
                            Value::Array(results)
                        }
                        Err(e) => Value::String(format!("Error: {}", e)),
                    }
                } else {
                    Value::String("Error: DB not open".to_string())
                }
            }
        });
    }
}
