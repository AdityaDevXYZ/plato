use std::sync::Arc;
use eventbus::{EventPublisher, EventMessage, Event};
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    async fn fetch_unpublished(&self, limit: i64) -> Result<Vec<(Uuid, String, String)>, String>;
    async fn mark_published(&self, id: Uuid) -> Result<(), String>;
    async fn mark_failed(&self, id: Uuid, reason: &str) -> Result<(), String>;
    async fn move_to_dlq(&self, id: Uuid, reason: &str) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct RawJsonEvent { pub evt_type: String, pub payload: String }
impl Event for RawJsonEvent {
    fn event_type(&self) -> &'static str { "RawJsonEvent" }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

pub struct OutboxWorker {
    repo: Arc<dyn OutboxRepository>,
    publisher: Arc<dyn EventPublisher>,
}

impl OutboxWorker {
    pub fn new(repo: Arc<dyn OutboxRepository>, publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn run_once(&self) -> Result<usize, String> {
        let messages = self.repo.fetch_unpublished(50).await?;
        let count = messages.len();

        for (id, evt_type, payload) in messages {
            let msg = EventMessage::new(Box::new(RawJsonEvent { evt_type, payload }), Uuid::new_v4(), "outbox");
            match self.publisher.publish(vec![msg]) {
                Ok(_) => { 
                    self.repo.mark_published(id).await.ok(); 
                    metrics::counter!("outbox_published_total").increment(1);
                },
                Err(e) => {
                    let _ = self.repo.mark_failed(id, &e.to_string()).await;
                    let _ = self.repo.move_to_dlq(id, &e.to_string()).await;
                    metrics::counter!("dlq_messages_total").increment(1);
                }
            }
        }
        Ok(count)
    }
}
