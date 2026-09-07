use eventbus::Event;
use std::any::Any;
use types::ReleaseId;

macro_rules! define_event {
    ($name:ident) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub release_id: ReleaseId,
        }
        impl Event for $name {
            fn event_type(&self) -> &'static str { stringify!($name) }
            fn as_any(&self) -> &dyn Any { self }
        }
    };
}

define_event!(ReleaseCreated);
define_event!(ReleaseValidated);
define_event!(ReleaseSubmitted);
define_event!(ReleaseApproved);
define_event!(ReleaseAssured);
define_event!(ReleaseReady);
define_event!(ReleaseDeploymentStarted);
define_event!(ReleaseCompleted);
define_event!(ReleaseFailed);
define_event!(ReleaseCancelled);
