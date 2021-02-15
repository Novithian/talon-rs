use cgmath::SquareMatrix;

use crate::renderer::camera::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    // Can't use cgmath with bytemuck directly
    model_view_projection: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model_view_projection: cgmath::Matrix4::identity().into(),
        }
    }
}

pub struct UniformStaging {
    pub camera: Camera,
    pub model_rotation: cgmath::Deg<f32>,
}

impl UniformStaging {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            model_rotation: cgmath::Deg(0.0),
        }
    }
    pub fn update_uniforms(&self, uniforms: &mut Uniforms) {
        uniforms.model_view_projection = (OPENGL_TO_WGPU_MATRIX
            * self.camera.build_view_projection_matrix()
            * cgmath::Matrix4::from_angle_z(self.model_rotation))
        .into();
    }

    pub fn set_camera_aspect(&mut self, aspect: f32) {
        self.camera.aspect = aspect;
    }
}
