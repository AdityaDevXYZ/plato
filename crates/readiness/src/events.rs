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

define_event!(DeploymentReadinessStarted { release_id: ReleaseId, report_id: Uuid });
define_event!(DeploymentReadinessCompleted { release_id: ReleaseId, report_id: Uuid, decision: String });
define_event!(ReleaseReady { release_id: ReleaseId, report_id: Uuid });
define_event!(DeploymentNotReady { release_id: ReleaseId, report_id: Uuid, reason: String });
