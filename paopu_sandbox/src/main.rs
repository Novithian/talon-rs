
use paopu_rs::{
    core::application::*,
    renderer::renderer::*,
};
fn main() {
    let mut app = Application::default();
    app 
        .add_module::<Renderer>(Renderer::default());
    app.run();
}
