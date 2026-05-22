use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use futures_util::stream::StreamExt;

pub struct QueueHost {
    channel: lapin::Channel,
}

impl QueueHost {
    pub async fn new(url: &str) -> Result<Self, String> {
        let conn = Connection::connect(url, ConnectionProperties::default())
            .await
            .map_err(|e| e.to_string())?;
        let channel = conn.create_channel().await.map_err(|e| e.to_string())?;
        Ok(QueueHost { channel })
    }

    pub async fn publish(&self, queue: &str, payload: Vec<u8>) -> Result<(), String> {
        self.channel.basic_publish(
            "",
            queue,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await
        .map_err(|e| e.to_string())?
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn consume(&self, queue: &str) -> Result<Vec<u8>, String> {
        let mut consumer = self.channel.basic_consume(
            queue,
            "navescript-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| e.to_string())?;

        if let Some(delivery) = consumer.next().await {
            let delivery = delivery.map_err(|e| e.to_string())?;
            delivery.ack(BasicAckOptions::default()).await.map_err(|e| e.to_string())?;
            Ok(delivery.data)
        } else {
            Err("No messages".to_string())
        }
    }
}
