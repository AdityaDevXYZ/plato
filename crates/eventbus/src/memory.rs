use crate::core::{EventMessage, EventPublisher, EventDispatcher, EventHandler, EventError};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/// An in-memory, synchronous event bus for development.
pub struct InMemoryEventBus {
    handlers: RwLock<HashMap<String, Vec<Arc<dyn EventHandler>>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) {
        let mut map = self.handlers.write().unwrap();
        map.entry(event_type.to_string()).or_default().push(handler);
    }
}

impl EventDispatcher for InMemoryEventBus {
    fn dispatch(&self, event: &EventMessage) -> Result<(), EventError> {
        let map = self.handlers.read().unwrap();
        if let Some(handlers) = map.get(&event.event_type) {
            for handler in handlers {
                handler.handle(event)?;
            }
        }
        Ok(())
    }
}

impl EventPublisher for InMemoryEventBus {
    fn publish(&self, events: Vec<EventMessage>) -> Result<(), EventError> {
        for event in events {
            self.dispatch(&event)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Event;
    use std::any::Any;
    use uuid::Uuid;

    #[derive(Debug)]
    struct TestEvent;
    impl Event for TestEvent {
        fn event_type(&self) -> &'static str { "TestEvent" }
        fn as_any(&self) -> &dyn Any { self }
    }

    struct TestHandler {
        called: std::sync::atomic::AtomicBool,
    }
    impl EventHandler for TestHandler {
        fn handle(&self, _event: &EventMessage) -> Result<(), EventError> {
            self.called.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn test_dispatch_and_correlation() {
        let bus = InMemoryEventBus::new();
        let handler = Arc::new(TestHandler { called: std::sync::atomic::AtomicBool::new(false) });
        
        bus.subscribe("TestEvent", handler.clone());
        
        let correlation_id = Uuid::new_v4();
        let msg = EventMessage::new(Box::new(TestEvent), correlation_id, "test");
        assert_eq!(msg.correlation_id, correlation_id);

        bus.publish(vec![msg]).unwrap();
        assert!(handler.called.load(std::sync::atomic::Ordering::SeqCst));
    }
}
