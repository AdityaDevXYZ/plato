use eventbus::Event;
use std::any::Any;
use types::ReleaseId;
use uuid::Uuid;

macro_rules! define_event {
    ($name:ident { $($field:ident: $type:ty),* }) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(pub $field: $type),*
        }
        impl Event for $name {
            fn event_type(&self) -> &'static str { stringify!($name) }
            fn as_any(&self) -> &dyn Any { self }
        }
    };
}

define_event!(DeploymentStarted { session_id: Uuid, release_id: ReleaseId });
define_event!(DeploymentProgressUpdated { session_id: Uuid, step: String });
define_event!(DeploymentRetrying { session_id: Uuid, step: String, attempt: u32 });
define_event!(DeploymentSucceeded { session_id: Uuid, release_id: ReleaseId });
define_event!(DeploymentFailed { session_id: Uuid, release_id: ReleaseId, reason: String });
define_event!(DeploymentRollbackStarted { session_id: Uuid, reason: String });
define_event!(DeploymentRollbackCompleted { session_id: Uuid, success: bool });
