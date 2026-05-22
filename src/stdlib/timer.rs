use crate::runtime::{NaveRuntime, Value};
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    let timers: Arc<Mutex<HashMap<u64, tokio::task::JoinHandle<()>>>> = Arc::new(Mutex::new(HashMap::new()));
    let next_id = Arc::new(Mutex::new(0u64));

    let t_timers = timers.clone();
    let t_next_id = next_id.clone();
    rt.register_async("setTimeout", move |args| {
        let timers = t_timers.clone();
        let next_id = t_next_id.clone();
        async move {
            let delay_ms = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(0);
            let id = {
                let mut id_val = next_id.lock().unwrap();
                let id = *id_val;
                *id_val += 1;
                id
            };
            
            let handle = tokio::spawn(async move {
                sleep(Duration::from_millis(delay_ms)).await;
                println!("[Timer] Timeout {} fired", id);
            });

            timers.lock().unwrap().insert(id, handle);
            Value::Number(id as f64)
        }
    });

    let i_timers = timers.clone();
    let i_next_id = next_id.clone();
    rt.register_async("setInterval", move |args| {
        let timers = i_timers.clone();
        let next_id = i_next_id.clone();
        async move {
            let delay_ms = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(1000);
            let id = {
                let mut id_val = next_id.lock().unwrap();
                let id = *id_val;
                *id_val += 1;
                id
            };
            
            let handle = tokio::spawn(async move {
                loop {
                    sleep(Duration::from_millis(delay_ms)).await;
                    println!("[Timer] Interval {} fired", id);
                }
            });

            timers.lock().unwrap().insert(id, handle);
            Value::Number(id as f64)
        }
    });

    let c_timers = timers.clone();
    rt.register_async("clearTimer", move |args| {
        let timers = c_timers.clone();
        async move {
            let id = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(0);
            if let Some(handle) = timers.lock().unwrap().remove(&id) {
                handle.abort();
                println!("[Timer] Timer {} cleared", id);
            }
            Value::Nil
        }
    });

    // Aliases
    let ct_timers = timers.clone();
    rt.register_async("clearTimeout", move |args| {
        let timers = ct_timers.clone();
        async move {
            let id = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(0);
            if let Some(handle) = timers.lock().unwrap().remove(&id) {
                handle.abort();
            }
            Value::Nil
        }
    });

    let ci_timers = timers.clone();
    rt.register_async("clearInterval", move |args| {
        let timers = ci_timers.clone();
        async move {
            let id = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(0);
            if let Some(handle) = timers.lock().unwrap().remove(&id) {
                handle.abort();
            }
            Value::Nil
        }
    });
}
