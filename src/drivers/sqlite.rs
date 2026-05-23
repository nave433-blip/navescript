use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

pub struct SqliteHost {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteHost {
    pub async fn new(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        Ok(SqliteHost { conn: Arc::new(Mutex::new(conn)) })
    }

    pub async fn execute(&self, query: &str) -> Result<u64, String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(query, []).map(|n| n as u64).map_err(|e| e.to_string())
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Vec<String>>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
        let column_count = stmt.column_count();
        
        let rows = stmt.query_map([], |row| {
            let mut res = Vec::new();
            for i in 0..column_count {
                let val: String = row.get_unwrap(i);
                res.push(val);
            }
            Ok(res)
        }).map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| e.to_string())?);
        }
        Ok(results)
    }
}
