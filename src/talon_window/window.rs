use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use futures::executor::block_on;

use std::any::Any;

use crate::{
    core::application::Application, 
    core::module::Module, 
    //core::application_event::ApplicationEvent,
    core::application_events::*,
    renderer::state_descriptor::StateDescriptor,
    renderer::renderer::Renderer,
};

pub struct Window {
    title: String,
    //pub window: Option<winit::window::Window>,
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
            title: String::from("Talon-rs Application"),
     //       window: None,
        }
    }
}

impl Window {
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
    /*fn window_id(&self) -> &winit::window::WindowId {
        &self.window.unwrap().id()
    }*/
}

pub fn run(app: Application) {
    winit_run(app, EventLoop::new());
}

pub fn winit_run(mut app: Application, event_loop: EventLoop<()>) {
    //let window = app.add_module(//app.get_module::<Window>().unwrap();
    //app.add_module(Window::default());
    let window =  WindowBuilder::new().build(&event_loop).unwrap();
    let actual_wid = window.id();
    {
        window.set_title( app.get_module_mut::<Window>().unwrap().get_title() );
    }
    let state = block_on( StateDescriptor::new(&window));
    app.send_event(ApplicationEventID::RendererSetup, Box::new(RendererSetup{ state_descriptor: Some(state) }));
   
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the os hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == actual_wid => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        // resize event
                        app.send_event(
                            ApplicationEventID::WindowResize, 
                            Box::new(WindowResize { 
                                width: physical_size.width,
                                height: physical_size.height,
                            })
                        );
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // resize event
                        app.send_event(
                            ApplicationEventID::WindowResize, 
                            Box::new(WindowResize { 
                                width: new_inner_size.width,
                                height: new_inner_size.height,
                            })
                        );
                    }
                    _ => (),
                }
            }
            Event::MainEventsCleared => {
                // Application update code

                // Queue redraw requested event
                //get window from app mods.request_redraw();
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application
                match app.get_module_mut::<Renderer>().unwrap().render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => {
                        app.send_event(
                            ApplicationEventID::WindowResize, 
                            Box::new(WindowResize { 
                                width: 0,
                                height: 0,
                            })
                        )
                    },
                    // The system is out of memory, just quit.
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors(Outdates, Timeout) should be resolved by the next frame.
                    Err(e) => eprintln!("{:?}", e),
                    
                }

            }
            _ => (),
        }
    });
}
