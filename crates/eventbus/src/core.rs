use uuid::Uuid;
use time::OffsetDateTime;
use std::collections::HashMap;
use std::any::Any;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Handler error: {0}")]
    HandlerFailed(String),
}

/// Base trait for all events in the system.
pub trait Event: std::fmt::Debug + Send + Sync + 'static {
    fn event_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

/// The common event abstraction wrapper.
#[derive(Debug)]
pub struct EventMessage {
    pub event_id: Uuid,
    pub correlation_id: Uuid,
    pub occurred_at: OffsetDateTime,
    pub event_type: String,
    pub version: u32,
    pub source: String,
    pub metadata: HashMap<String, String>,
    pub payload: Box<dyn Event>,
}

impl EventMessage {
    pub fn new(payload: Box<dyn Event>, correlation_id: Uuid, source: impl Into<String>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            correlation_id,
            occurred_at: OffsetDateTime::now_utc(),
            event_type: payload.event_type().to_string(),
            version: 1,
            source: source.into(),
            metadata: HashMap::new(),
            payload,
        }
    }
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, events: Vec<EventMessage>) -> Result<(), EventError>;
}

pub trait EventDispatcher: Send + Sync {
    fn dispatch(&self, event: &EventMessage) -> Result<(), EventError>;
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &EventMessage) -> Result<(), EventError>;
}
