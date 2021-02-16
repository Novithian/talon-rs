use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};


use std::any::Any;

use crate::{
    core::application::Application,
    core::module::Module,
};

pub struct Window {
    title: String,
}

impl Module for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn build(&self, app: &mut Application) {
        app.set_loop_function(run);
    }

}

impl Default for Window {
    fn default() -> Self {
        Window {
            title: String::from("Paopu-rs Application"),
        }
    }
}

impl Window {
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
}

pub fn run(app: Application) {
    winit_run(app, EventLoop::new());
}

pub fn winit_run(mut app: Application, event_loop: EventLoop<()>) {
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let actual_wid = window.id();
    {
        window.set_title(app.get_module_mut::<Window>().unwrap().get_title());
    }

    // Request that the State Descriptor be created
    app.create_state( &window );

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the os hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == actual_wid => {
                // Only triggered if termination is requested in another module.
                if app.requested_termination {
                    *control_flow = ControlFlow::Exit;
                // Else check for non-window inputs
                }else if !app.update_input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            // resize event
                            app.window_resize(
                                physical_size.width,
                                physical_size.height,
                            )
                            
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // resize event
                            app.window_resize(
                                new_inner_size.width,
                                new_inner_size.height,
                            )
                        },
                        _ => (),
                    }
                }
            }
            Event::MainEventsCleared => {
                // Application update code
                app.update();
                // Queue redraw requested event
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application
                if app.render() {
                    // If there was a fatal error, then request a shutdown.
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
