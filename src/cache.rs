use moka::future::Cache;
use std::time::Duration;

pub struct CacheHost {
    cache: Cache<String, String>,
}

impl CacheHost {
    pub fn new(max_capacity: u64, ttl_secs: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();
        CacheHost { cache }
    }

    pub async fn get(&self, key: String) -> Result<String, String> {
        self.cache.get(&key)
            .ok_or_else(|| "Key not found".to_string())
    }

    pub async fn set(&self, key: String, value: String) {
        self.cache.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: String) {
        self.cache.invalidate(&key).await;
    }
}
