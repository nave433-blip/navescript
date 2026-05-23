use super::postgres::PostgresHost;
use super::mysql::MySqlHost;
use super::redis::RedisHost;
use super::sqlite::SqliteHost;
use super::mongodb::MongoHost;

pub enum DbConnection {
    Postgres(PostgresHost),
    MySql(MySqlHost),
    Redis(RedisHost),
    Sqlite(SqliteHost),
    Mongo(MongoHost),
}

pub struct DbHost {
    conn: DbConnection,
}

impl DbHost {
    pub async fn new(url: &str, db_type: &str) -> Result<Self, String> {
        let conn = match db_type {
            "postgres" => DbConnection::Postgres(PostgresHost::new(url).await?),
            "mysql" => DbConnection::MySql(MySqlHost::new(url).await?),
            "redis" => DbConnection::Redis(RedisHost::new(url).await?),
            "sqlite" => DbConnection::Sqlite(SqliteHost::new(url).await?),
            "mongodb" => DbConnection::Mongo(MongoHost::new(url).await?),
            _ => return Err("Unsupported DB type".to_string()),
        };
        Ok(DbHost { conn })
    }

    pub async fn execute(&self, query: &str) -> Result<u64, String> {
        match &self.conn {
            DbConnection::Postgres(c) => c.execute(query).await,
            DbConnection::MySql(c) => c.execute(query).await,
            DbConnection::Sqlite(c) => c.execute(query).await,
            _ => Err("Execute not supported on this connection".to_string()),
        }
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Vec<String>>, String> {
        match &self.conn {
            DbConnection::Postgres(c) => c.query(query).await,
            DbConnection::MySql(c) => c.query(query).await,
            DbConnection::Redis(c) => {
                let res = c.get(query.to_string()).await?;
                Ok(vec![vec![res]])
            }
            DbConnection::Sqlite(c) => c.query(query).await,
            DbConnection::Mongo(c) => c.query(query).await,
        }
    }
}
