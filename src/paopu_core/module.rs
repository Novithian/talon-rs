use std::any::Any;

use crate::core::application::Application;

pub trait Module: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn build(&self, app: &mut Application);

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn print_name(&self) {
        println!("{}", self.name());
    }
}
