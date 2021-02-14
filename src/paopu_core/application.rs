use std::{any::TypeId, collections::HashMap};

use crate::core::{application_event::ApplicationEvent, application_events::*, module::Module};

use crate::renderer::renderer::Renderer;

// -------------------------------------------------------------------------------
//                      - Application -
// -------------------------------------------------------------------------------

pub struct Application {
    modules: HashMap<TypeId, Box<dyn Module>>,
    loop_function: Box<dyn Fn(Application)>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            modules: HashMap::new(),
            loop_function: Box::new(no_loop),
        }
    }
}

impl Application {
    pub fn run(mut self) {
        let replaced_loop = std::mem::replace(&mut self.loop_function, Box::new(no_loop));
        (replaced_loop)(self);
    }
    pub fn set_loop_function(
        &mut self,
        loop_function: impl Fn(Application) + 'static,
    ) -> &mut Self {
        self.loop_function = Box::new(loop_function);
        self
    }
    pub fn update(&mut self) {
        // Update shit
    }

    // Modules
    pub fn add_module<T>(&mut self, module: T) -> &mut Self
    where
        T: Module,
    {
        module.build(self);
        self.modules.insert(module.type_id(), Box::new(module));
        self
    }
    pub fn list_module(&mut self) {
        println!("[Application]: Modules Enabled:");
        for (_id, module) in self.modules.iter_mut() {
            module.print_name();
        }
    }
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

fn no_loop(mut app: Application) {
    app.update();
}
