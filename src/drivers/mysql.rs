use sqlx::{mysql::MySqlPool, MySql, Pool, Row};

pub struct MySqlHost {
    pool: Pool<MySql>,
}

impl MySqlHost {
    pub async fn new(url: &str) -> Result<Self, String> {
        let pool = MySqlPool::connect(url).await.map_err(|e| e.to_string())?;
        Ok(MySqlHost { pool })
    }

    pub async fn execute(&self, query: &str) -> Result<u64, String> {
        let result = sqlx::query(query).execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(result.rows_affected())
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Vec<String>>, String> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await.map_err(|e| e.to_string())?;
        
        let mut results = Vec::new();
        for row in rows {
            let mut row_data = Vec::new();
            for i in 0..row.len() {
                // Simplified: convert to string
                let val: String = row.try_get(i).unwrap_or_else(|_| "NULL".to_string());
                row_data.push(val);
            }
            results.push(row_data);
        }
        Ok(results)
    }
}
