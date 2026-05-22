use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub struct PostgresHost {
    pool: Pool,
}

impl PostgresHost {
    pub async fn new(config_str: &str) -> Result<Self, String> {
        let mut cfg = Config::new();
        cfg.url = Some(config_str.to_string());
        
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| e.to_string())?;
            
        Ok(PostgresHost { pool })
    }

    pub async fn execute(&self, query: &str) -> Result<u64, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        client.execute(query, &[]).await.map_err(|e| e.to_string())
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Vec<String>>, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let rows = client.query(query, &[]).await.map_err(|e| e.to_string())?;
        
        let mut results = Vec::new();
        for row in rows {
            let mut row_data = Vec::new();
            for i in 0..row.len() {
                // Simplified: convert everything to string
                let val: String = row.get(i);
                row_data.push(val);
            }
            results.push(row_data);
        }
        Ok(results)
    }
}
