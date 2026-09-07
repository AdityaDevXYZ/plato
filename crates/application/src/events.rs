use eventbus::Event;
use std::any::Any;
use types::ReleaseId;

/// Application Event representing orchestration facts.
#[derive(Debug, Clone)]
pub struct DeploymentRequested {
    pub release_id: ReleaseId,
}

impl Event for DeploymentRequested {
    fn event_type(&self) -> &'static str { "DeploymentRequested" }
    fn as_any(&self) -> &dyn Any { self }
}
