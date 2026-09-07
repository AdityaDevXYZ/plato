use eventbus::Event;
use std::any::Any;
use uuid::Uuid;
use types::{MissionId, ReleaseId};

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

define_event!(DeploymentPlanCreated { plan_id: Uuid, mission_id: MissionId, release_id: ReleaseId });
define_event!(DeploymentPlanUpdated { plan_id: Uuid });
define_event!(PlanningFailed { mission_id: MissionId, release_id: ReleaseId, reason: String });
