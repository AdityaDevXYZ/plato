use eventbus::Event;
use std::any::Any;
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

define_event!(ResourceUpdated { entity_id: Uuid });
define_event!(ResourceThresholdExceeded { entity_id: Uuid, resource_type: String, value: f32, threshold: f32 });
define_event!(GroundStationCapacityChanged { ground_station_id: Uuid, occupancy_pct: f32 });
