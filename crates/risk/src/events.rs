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

define_event!(RiskAssessmentCompleted { report_id: Uuid, plan_id: Uuid, overall_score: u8 });
define_event!(RiskThresholdExceeded { report_id: Uuid, plan_id: Uuid, risk_level: String });
