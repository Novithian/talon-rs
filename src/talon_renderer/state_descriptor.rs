use winit::window::Window;

use wgpu::util::DeviceExt;

use crate::renderer::{
    texture::Texture,
    vertex_buffer_descriptor::Vertex,
    camera::{ Camera, CameraController },
    uniforms::{ Uniforms, UniformStaging },
};

const VERTICES: &[Vertex] = &[
    // Top left
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
    // Top Right
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
    // Bottom Right
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
    // Bottom Left
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

// -------------------------------------------------------
//              - State Descriptor -
// -------------------------------------------------------
              
pub struct StateDescriptor {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub clear_color: wgpu::Color,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indicies: u32,
    pub camera_controller: CameraController,
    uniform_staging: UniformStaging,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    diffuse_texture: Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl StateDescriptor {
    // Creating some of the wgpu types requires async
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace Path
            )
            .await
            .unwrap();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        let diffuse_bytes = include_bytes!("../../assets/s_mob_01_idle.png");
        let diffuse_texture =
            Texture::from_bytes(&device, &queue, diffuse_bytes, "mob sprite").unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            //sample_type: wgpu::TextureSampleType::Uint,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: false,
                        },
                        count: None,
                    },
                ],
                label: Some("Texture Bind Group Layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("Diffuse Bind Group"),
        });

        let camera = Camera {
            // Position the camera 1 unit up and 2 units back
            eye: (0.0, 0.0, 10.0).into(),
            // Look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // Up direction
            up: cgmath::Vector3::unit_y(),
            aspect: swap_chain_descriptor.width as f32 / swap_chain_descriptor.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(0.2);

        let mut uniforms = Uniforms::new();
        let uniform_staging = UniformStaging::new(camera);
        uniform_staging.update_uniforms(&mut uniforms);
        //uniforms.update_view_projection(&camera);

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout( &wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        // Indicates wehterh this buffer will change size or not. 
                        // Useful for storing an array of things in out uniform.
                        has_dynamic_offset: false, 
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("Uniform Bind Group Layout"),
        });

        let uniform_bind_group = device.create_bind_group( &wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        // Base offset of the buffer. For bindings with dynamic == true, this
                        // offset will be added to the dynamic offset provided in
                        // [RenderPass::set_bind_group].
                        // The offset has to be aling to [BIND_BUFFER_ALIGNMENT].
                        offset: 0,
                        // Size of the binding, or [None] for using the rest of the buffer.
                        size: None,
                    },
                }
            ],
            label: Some("Uniform Bind Group"),
        });

        let clear_color = wgpu::Color::BLACK;

        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        let render_pipline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &uniform_bind_group_layout,

                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Core Pipeline"),
            layout: Some(&render_pipline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[Vertex::descriptor()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: swap_chain_descriptor.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        let num_indicies = INDICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indicies,
            camera_controller,
            uniform_staging,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            diffuse_texture,
            diffuse_bind_group,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.uniform_staging.camera);
        //self.uniform_staging.model_rotation += cgmath::Deg(2.0);
        self.uniform_staging.update_uniforms(&mut self.uniforms);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.swap_chain_descriptor.width = new_size.width;
        self.swap_chain_descriptor.height = new_size.height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
        self.uniform_staging
            .set_camera_aspect(
                self.swap_chain_descriptor.width as f32
                / self.swap_chain_descriptor.height as f32
            );
    }
}
