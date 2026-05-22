use mongodb::{Client, Collection, bson::{Document}};
use futures_util::stream::StreamExt;

pub struct MongoHost {
    db: mongodb::Database,
}

impl MongoHost {
    pub async fn new(url: &str, db_name: &str) -> Result<Self, String> {
        let client = Client::with_uri_str(url).await.map_err(|e| e.to_string())?;
        let db = client.database(db_name);
        Ok(MongoHost { db })
    }

    pub async fn find(&self, collection: &str, filter_json: &str) -> Result<Vec<String>, String> {
        let coll: Collection<Document> = self.db.collection(collection);
        let filter: Document = serde_json::from_str(filter_json).map_err(|e| e.to_string())?;
        
        let mut cursor = coll.find(filter, None).await.map_err(|e| e.to_string())?;
        let mut results = Vec::new();
        while let Some(doc) = cursor.next().await {
            let d = doc.map_err(|e| e.to_string())?;
            results.push(d.to_string());
        }
        Ok(results)
    }

    pub async fn insert(&self, collection: &str, doc_json: &str) -> Result<String, String> {
        let coll: Collection<Document> = self.db.collection(collection);
        let doc: Document = serde_json::from_str(doc_json).map_err(|e| e.to_string())?;
        
        let result = coll.insert_one(doc, None).await.map_err(|e| e.to_string())?;
        Ok(result.inserted_id.to_string())
    }
}
