use mongodb::{Client, Collection, bson::{Document}};
use futures_util::stream::StreamExt;

pub struct MongoHost {
    db: mongodb::Database,
}

impl MongoHost {
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = Client::with_uri_str(url).await.map_err(|e| e.to_string())?;
        // Default to 'navescript' database if not specified in URL
        let db = client.database("navescript");
        Ok(MongoHost { db })
    }

    pub async fn execute(&self, query_json: &str) -> Result<u64, String> {
        // For Mongo, 'execute' could mean an insertion or update
        let doc: Document = serde_json::from_str(query_json).map_err(|e| e.to_string())?;
        let coll_name = doc.get_str("collection").map_err(|_| "Missing 'collection' field".to_string())?;
        let coll: Collection<Document> = self.db.collection(coll_name);
        
        if let Ok(insert_doc) = doc.get_document("insert") {
            coll.insert_one(insert_doc.clone(), None).await.map_err(|e| e.to_string())?;
            Ok(1)
        } else {
            Err("Unsupported execute command for Mongo".to_string())
        }
    }

    pub async fn query(&self, query_json: &str) -> Result<Vec<Vec<String>>, String> {
        let doc: Document = serde_json::from_str(query_json).map_err(|e| e.to_string())?;
        let coll_name = doc.get_str("collection").map_err(|_| "Missing 'collection' field".to_string())?;
        let filter = doc.get_document("filter").unwrap_or(&Document::new()).clone();
        
        let coll: Collection<Document> = self.db.collection(coll_name);
        let mut cursor = coll.find(filter, None).await.map_err(|e| e.to_string())?;
        
        let mut results = Vec::new();
        while let Some(res) = cursor.next().await {
            let d = res.map_err(|e| e.to_string())?;
            results.push(vec![d.to_string()]);
        }
        Ok(results)
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
