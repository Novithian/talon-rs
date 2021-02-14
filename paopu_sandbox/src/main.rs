
use paopu_rs::{
    core::application::*,
    window::window::*,
    renderer::renderer::*,
};
fn main() {
    let mut app = Application::default();
    app 
        .add_module::<Window>(Window::default())
        .add_module::<Renderer>(Renderer::default());
    app.run();
}
