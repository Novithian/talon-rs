use crate::core::application_event::*;
use crate::renderer::state_descriptor::StateDescriptor;

use std::any::Any;

pub enum ApplicationEventID {
    WindowResize,
    RendererSetup,
    Render
}

// -----------------------------------------------
//          - Window Events -
// -----------------------------------------------
pub struct WindowResize {
    pub width: u32,
    pub height: u32,
}

impl ApplicationEvent for WindowResize {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_id(&self) -> Option<ApplicationEventID> {
        Some (ApplicationEventID::WindowResize )
    }
}
// -----------------------------------------------
//          - Renderer Events -
// -----------------------------------------------

pub struct RendererSetup {
    pub state_descriptor: Option<StateDescriptor>,
}

impl ApplicationEvent for RendererSetup {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_id(&self) -> Option<ApplicationEventID> {
        Some (ApplicationEventID::RendererSetup )
    }
}

impl Default for RendererSetup {
    fn default() -> Self {
        RendererSetup {
            state_descriptor: None,
        }
    }
}

