use crate::renderer::state_descriptor::StateDescriptor;

pub enum PaopuEvent {
    // -----------------------------------------------
    //          - Window Events -
    // -----------------------------------------------
    WindowResize { id: i8, width: u32, height: u32 },
    // -----------------------------------------------
    //          - Renderer Events -
    // -----------------------------------------------
    RendererSetup { id: i8, state: Option<StateDescriptor> },
}

