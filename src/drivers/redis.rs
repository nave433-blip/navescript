use redis::{AsyncCommands, Client};

pub struct RedisHost {
    connection: redis::aio::MultiplexedConnection,
}

impl RedisHost {
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = Client::open(url).map_err(|e| e.to_string())?;
        let connection = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        Ok(RedisHost { connection })
    }

    pub async fn get(&self, key: String) -> Result<String, String> {
        let mut conn = self.connection.clone();
        conn.get(key).await.map_err(|e| e.to_string())
    }

    pub async fn set(&self, key: String, value: String) -> Result<(), String> {
        let mut conn = self.connection.clone();
        conn.set(key, value).await.map_err(|e| e.to_string())
    }
}
