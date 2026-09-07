use eventbus::Event;
use std::any::Any;
use uuid::Uuid;
use types::ReleaseId;

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

define_event!(CoordinationStarted { session_id: Uuid, plan_id: Uuid, release_id: ReleaseId });
define_event!(WaveStarted { session_id: Uuid, wave_id: Uuid, target_satellites: Vec<Uuid> });
define_event!(WaveCompleted { session_id: Uuid, wave_id: Uuid });
define_event!(WaveFailed { session_id: Uuid, wave_id: Uuid, reason: String });
define_event!(CoordinationCompleted { session_id: Uuid, plan_id: Uuid });
