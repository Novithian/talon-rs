use std::{any::TypeId, collections::HashMap};

use futures::executor::block_on;

use glfw;
use glfw::{
    Action, 
    Context, 
    Key
};

use crate::core::{
    application_event::ApplicationEvent, 
    application_events::*, 
    module::Module
};
use crate::renderer::{
    renderer::Renderer,
    state_descriptor::StateDescriptor,
};
use crate::window::window::Window;

// -------------------------------------------------------------------------------
//                      - Application -
// -------------------------------------------------------------------------------

pub struct Application {
    window: Window,
    modules: HashMap<TypeId, Box<dyn Module>>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            window: Window::default(),
            modules: HashMap::new(),
        }
    }
}

impl Application {
    
    /// Called to start the main game loop.
    /// Note: This takes ownership of self from the client.
    /// 
    /// [`Update`]: #trait.Update
    /// [`Render`]: #trait.Render
    pub fn run(mut self) {

        // Create a glfw token
        let mut gl = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        println!("Vulkan supported: {:?}", gl.vulkan_supported());

        let (mut gl_window, events) = 
            gl
                .create_window(
                    1280, 
                    720, 
                    "Paopu-rs Application",
                    glfw::WindowMode::Windowed
                ).expect("[Application]: Failed to create an Window!");

        gl_window.set_key_polling(true);
        gl_window.make_current();
        

        println!("After window creation");

        // Create the State Descriptor 
        let state_descriptor = block_on(StateDescriptor::new(&gl_window));


        println!("Before context");

        // Set [`Paopu::Window`]'s context member with the glfw_window
        self.window.context = Some ( gl_window );
        

        // Check to see if there is a [`Paopu::Renderer`]  
        self.get_module_mut::<Renderer>().unwrap().set_state(state_descriptor);
        
        println!("After SC");

        let mut exit_application = false;
        
        // Application Loop
        while !exit_application {
            gl.poll_events();
            for(_, event) in glfw::flush_messages(&events) {
                // Handle the events
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        exit_application = true;
                        break;
                    },
                    _ => (),
                }
            }

            // Update
            self.update();


            // Render
            if self.get_module_mut::<Renderer>().unwrap().update() {
                exit_application = true;
            }
            
        }

        // Clean up
        //self.window.close();


    }
    
    /// Called once every frame.
    pub fn update(&mut self) {
        // Update shit
    }

    /// Adds a module of type `T` to the application.
    /// Returns a mutable reference to `Self` 
    /// Note: `Application takes ownership of `module`!
    ///
    /// # Arguments
    ///
    /// * `module` - The module that the user wishes to add to 
    /// the application.
    pub fn add_module<T>(&mut self, module: T) -> &mut Self
    where
        T: Module,
    {
        module.build(self);
        self.modules.insert(module.type_id(), Box::new(module));
        self
    }

    // TODO: Remove on release
    pub fn list_module(&mut self) {
        println!("[Application]: Modules Enabled:");
        for (_id, module) in self.modules.iter_mut() {
            module.print_name();
        }
    }

    /// Checks to see if `Application` has a module of type `T`
    /// and returns a mutable reference if found.
    /// Returns an `Option<&mut T>
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
    /// and returns a immutable reference if found.
    /// Returns an `Option<&T>
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
     
    //TODO: Revamp the event system
    pub fn send_event(
        &mut self,
        event_id: ApplicationEventID,
        mut app_event: Box<dyn ApplicationEvent>,
    ) {
        match event_id {
            ApplicationEventID::WindowResize => {
                let resize_event = app_event
                    .as_any_mut()
                    .downcast_mut::<WindowResize>()
                    .expect("[Application]: Application Event downcast failed!");
                self.get_module_mut::<Renderer>()
                    .unwrap()
                    .resize(resize_event.width, resize_event.height);
            }
            ApplicationEventID::RendererSetup => {
                let setup_event = app_event
                    .as_any_mut()
                    .downcast_mut::<RendererSetup>()
                    .expect("[Application]: Application Event downcast failed!");

                self.get_module_mut::<Renderer>()
                    .unwrap()
                    .set_state(std::mem::take(setup_event).state_descriptor.unwrap());
            }
            ApplicationEventID::Render => (),
            //_ => (),
        }
    }
}

