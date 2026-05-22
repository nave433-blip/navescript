use crate::polyglot::PolyglotBridge;
use tokio::task;

pub struct Task {
    pub lang: String,
    pub code: String,
}

pub struct BatchProcessor;

impl BatchProcessor {
    pub async fn run(tasks: Vec<Task>) -> Result<Vec<String>, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        let mut handles = vec![];

        for task_item in tasks {
            let bridge = bridge.clone(); // Note: PolyglotBridge needs Clone
            let handle = task::spawn(async move {
                bridge.eval_and_transmute(&task_item.lang, &task_item.code, crate::polyglot::ExecutionContext::default())
                    .map(|v| v.to_string())
                    .map_err(|e| e.to_string())
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            results.push(handle.await.map_err(|e| e.to_string())??);
        }

        Ok(results)
    }
}
