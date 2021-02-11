use std::any::Any;

use crate::{
    core::application::Application, 
    core::module::Module,
    renderer::state_descriptor::StateDescriptor,
};

// -------------------------------------------------------------------------
//              - Renderer -
// -------------------------------------------------------------------------

pub struct Renderer {
    state_descriptor: Option<StateDescriptor>,
}

impl Module for Renderer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // Needs a window to build properly
    fn build(&self, _app: &mut Application) {
        // TODO: After creating get module in ApplicationBuilder
        //let window_handle = &app.get_module::<Window>().unwrap().window;
        //self.setup(window_handle);
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer {
            state_descriptor: None,
        }
    }
}

impl Renderer {
    pub fn set_state(&mut self, state_descriptor: StateDescriptor) {
        self.state_descriptor = Some(state_descriptor);
    }

    pub fn resize(&mut self, desired_width: u32, desired_height: u32) {
        match self.state_descriptor.as_mut() {
            Some(sd) => {
                let width;
                let height;
                if desired_height == 0 && desired_width == 0 {
                    width = sd.size.width;
                    height = sd.size.height;
                }else{
                    width = desired_width;
                    height = desired_height;
                }
                sd.resize(
                    winit::dpi::PhysicalSize::<u32>::new(width, height)
                )
            },
            None => (),
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let state_desc = self.state_descriptor.as_mut().unwrap();
        let frame = state_desc.swap_chain.get_current_frame()?.output;
        let mut encoder = 
           state_desc 
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(state_desc.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&state_desc.render_pipeline);
            render_pass.set_vertex_buffer(0, state_desc.vertex_buffer.slice(..));
            render_pass.set_index_buffer(state_desc.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..state_desc.num_indicies, 0, 0..1);
        }

        // Submit will accept anything that implments IntoIter
        state_desc.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
