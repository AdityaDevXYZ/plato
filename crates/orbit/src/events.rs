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

define_event!(OrbitalStateUpdated { satellite_id: Uuid });
define_event!(PredictionGenerated { satellite_id: Uuid });
define_event!(CommunicationWindowOpened { satellite_id: Uuid, ground_station_id: Uuid });
define_event!(CommunicationWindowClosed { satellite_id: Uuid, ground_station_id: Uuid });
