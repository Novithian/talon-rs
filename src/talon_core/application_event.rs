use std::any::Any;

use crate::core::application_events::ApplicationEventID;

pub trait ApplicationEvent: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_id(&self) -> Option<ApplicationEventID>;
}
