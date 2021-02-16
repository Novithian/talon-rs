use std::{any::TypeId, collections::HashMap};

use winit::event::*;

use crate::core::module::Module;
use crate::input::input::*;
use crate::renderer::renderer::Renderer;

// -------------------------------------------------------------------------------
//                      - Application -
// -------------------------------------------------------------------------------

pub struct Application {
    modules: HashMap<TypeId, Box<dyn Module>>,
    loop_function: Box<dyn Fn(Application)>,

    pub requested_termination: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            modules: HashMap::new(),
            loop_function: Box::new(no_loop),

            requested_termination: false,
        }
    }
}

impl Application {
    /// Called to start the main game loop.
    /// Note: This takes ownership of self from the client.
    ///          
    pub fn run(mut self) {
        let replaced_loop = std::mem::replace(&mut self.loop_function, Box::new(no_loop));
        (replaced_loop)(self);
    }
    
    /// Sets the main event loop
    pub fn set_loop_function(
        &mut self,
        loop_function: impl Fn(Application) + 'static,
    ) -> &mut Self {
        self.loop_function = Box::new(loop_function);
        self
    }

    /// Adds a module of type `T` to the application.
    /// Returns a mutable reference to `Self`
    /// Note: `Application` takes ownership of `module`!
    ///
    /// # Arguments
    ///
    /// * `module` - The module that the user wishes to add 
    /// to the application.
    pub fn add_module<T>(&mut self, module: T) -> &mut Self
    where
        T: Module,
    {
        module.build(self);
        self.modules.insert(module.type_id(), Box::new(module));
        self
    }

    // TODO (devon): Remove on release
    pub fn list_module(&mut self) {
        println!("[Application]: Modules Enabled:");
        for (_id, module) in self.modules.iter_mut() {
            module.print_name();
        }
    }

    /// Checks to see if `Application` has a module of type `T`
    /// and returns a mutable reference if found.
    /// Returns `Option<&mut T>
    pub fn get_module_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Module,
    {
        Some(
            self.modules
                .get_mut(&TypeId::of::<T>())
                .unwrap()
                .as_any_mut()
                .downcast_mut::<T>()
                .expect("[Application]: Module downcast failed!"),
        )
    }

    /// Checks to see if `Application` has a module of type `T`
    /// and returns an immutable reference if found.
    /// Returns `Option<&T>
    pub fn get_module<T>(&mut self) -> Option<&T>
    where
        T: Module,
    {
        Some(
            self.modules
                .get(&TypeId::of::<T>())
                .unwrap()
                .as_any()
                .downcast_ref::<T>()
                .expect("[Application]: Module downcast failed!"),
        )
    }

    // ---------------------------------------------------------
    //                  Events
    // ---------------------------------------------------------
    
    /// Updates game logic.
    /// Called once every frame. 
    pub fn update(&mut self) {
        // Update shit
    }
    
    /// Renders the game entities to the window.
    /// Returns a boolean whether there was an error that 
    /// requires the application to shut down.
    pub fn render(&mut self) -> bool {
        let mut error = false;
        if let Some(r) = self.get_module_mut::<Renderer>() {
            error = r.update();
        }
        return error
    }

    /// Handles the window resize
    pub fn window_resize(&mut self, width: u32, height: u32) {
        if let Some(r) = self.get_module_mut::<Renderer>() {
            r.resize(width, height);
        }
    }

    /// Handles the window input events
    pub fn update_input(&mut self, event: &WindowEvent) -> bool {
        let input_event = INPUT.lock().unwrap().update(event);

        self.requested_termination = INPUT.lock().unwrap().get_key_pressed(PKeyCode::Escape);

        input_event
    }

    pub fn create_state(&mut self, window: &winit::window::Window ) {
        if let Some(r) = self.get_module_mut::<Renderer>() {
            r.create_state(window);
        }
    }

    
}

/// This is used as a default main event loop just in case none is specified.
fn no_loop(mut app: Application) {
    app.update();
}
