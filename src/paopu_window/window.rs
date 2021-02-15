// ------------------------------------------------------------------------------
//                      - Window -
// ------------------------------------------------------------------------------

pub struct Window {
    title: String,
    pub context: Option<glfw::Window>
}

impl Default for Window {
    fn default() -> Self {
        Window {
            title: String::from("Paopu-rs Application"),
            context: None,
        }
    }
}

impl Window {
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
    pub fn close(&mut self) {
        let context = self.context.take().unwrap();
        context.close();
    }
}
/*
pub fn run(app: Application) {
    winit_run(app, EventLoop::new());
}*/
/*
pub fn winit_run(mut app: Application, event_loop: EventLoop<()>) {
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let actual_wid = window.id();
    {
        window.set_title(app.get_module_mut::<Window>().unwrap().get_title());
    }
    let state = block_on(StateDescriptor::new(&window));
    app.send_event(
        ApplicationEventID::RendererSetup,
        Box::new(RendererSetup {
            state_descriptor: Some(state),
        }),
    );

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the os hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        let renderer = app.get_module_mut::<Renderer>().unwrap();

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == actual_wid => {
                if !renderer.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            // resize event
                            app.send_event(
                                ApplicationEventID::WindowResize,
                                Box::new(WindowResize {
                                    width: physical_size.width,
                                    height: physical_size.height,
                                }),
                            );
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // resize event
                            app.send_event(
                                ApplicationEventID::WindowResize,
                                Box::new(WindowResize {
                                    width: new_inner_size.width,
                                    height: new_inner_size.height,
                                }),
                            );
                        }
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
                renderer.update();
                match renderer.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => app.send_event(
                        ApplicationEventID::WindowResize,
                        Box::new(WindowResize {
                            width: 0,
                            height: 0,
                        }),
                    ),
                    // The system is out of memory, just quit.
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors(Outdates, Timeout) should be resolved by the next frame.
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    });
}*/
