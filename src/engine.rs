use std::iter;
use std::sync::Arc;

use crate::camera::{Camera, CameraState};
use crate::constants::WINDOW_TITLE;
use crate::shapes::{Circle, Shape};
use crate::texture::ImageTexture;
use crate::vertex::{TexturedVertex, Vertex, INDICES, VERTICES};
use cgmath::Vector3;
use wgpu::{
    util::DeviceExt, BlendComponent, Buffer, Features, Limits, PipelineLayoutDescriptor,
    RenderPipeline, ShaderModuleDescriptor, ShaderSource,
};
use wgpu::{MemoryHints, PipelineCompilationOptions, Trace};
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub async fn run() {
    env_logger::init();
    // Unwrap OK. Can only panic if not making event loop in the made thread.
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    render_pipeline: RenderPipeline,
    camera_state: CameraState,
    diffuse_bind_group: wgpu::BindGroup,
    // Normal state
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    // circle state
    circle_vertex_buffer: Buffer,
    circle_index_buffer: Buffer,
    circle_num_indices: u32,

    render_circle: bool,
}

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}


impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                window.request_redraw();
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        log::error!("Surface lost or outdated");
                        state.resize(state.size);
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                        log::error!("OutOfMemory");
                        event_loop.exit();
                    }
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface timeout")
                    }
                }
            }
            _ => (),
        }
    }
}

impl<'a> State<'a> {
    async fn new(window: Arc<Window>) -> State<'a> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::default(),
                trace: Trace::default(),
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);

        let camera = Camera {
            aspect: config.width as f32 / config.height as f32,
            ..Default::default()
        };
        let camera_state = CameraState::new(&device, camera);

        let diffuse_texture_bytes = include_bytes!("../assets/happy-tree.png");
        let diffuse_texture =
            ImageTexture::from_bytes(&device, &queue, diffuse_texture_bytes, "My First Texture")
                .unwrap();
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
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
            label: Some("diffuse_bind_group"),
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/textured.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &camera_state.bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("My Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[TexturedVertex::description()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: BlendComponent::REPLACE,
                        alpha: BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // A triangle is forwards if the verticies are counter clock wise.
                // If they are backwards then the triangle is not rendered (culled)
                // as per the cull_mode argument being set to Face::Back.
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_circle_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let circle = Circle::new([0.0, 0.0], 50, 0.5);
        let circle_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&circle.col_vertices([0.9, 0.5, 0.7, 1.0])),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let circle_indices = circle.indices();
        let circle_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&circle_indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let circle_num_indices = circle_indices.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            camera_state,
            diffuse_bind_group,
            vertex_buffer,
            index_buffer,
            num_indices,
            circle_vertex_buffer,
            circle_index_buffer,
            circle_num_indices,
            render_circle: false,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        // Returns a bool denoting if the event has been handled.
        // If it has been handled then we don't need to continue processing this event.
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(KeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.render_circle = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.camera_state.camera.eye += Vector3::new(-0.01, 0.01, 0.0);
        self.camera_state.camera.target += Vector3::new(-0.01, 0.01, 0.0);
        self.camera_state.update();
        self.queue.write_buffer(
            &self.camera_state.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_state.uniform]),
        );

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.6,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &self.camera_state.bind_group, &[]);
            // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            if self.render_circle {
                render_pass.set_vertex_buffer(0, self.circle_vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.circle_index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..self.circle_num_indices, 0, 0..1);
            } else {
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                // slice(..) specifies what part of the buffer to use (all of it).
                // If we wanted to only use part of a buffer we could provide another slice object.
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
