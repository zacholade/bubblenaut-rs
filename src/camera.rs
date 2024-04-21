use cgmath;
use wgpu::util::DeviceExt;

use crate::{camera_uniform::CameraUniform, constants::OPENGL_TO_WGPU_MATRIX};

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    // Aspect ration (width / heigth)
    pub aspect: f32,
    // field of view in the y direction
    pub fovy: f32,
    // Objects closer than this distance will not bw rendered.
    pub znear: f32,
    // Objects further than this distance will not be rendered
    pub zfar: f32,
}

impl Camera {
    pub fn projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // view matrix moves world to be at position and rotation of the camera.
        // Inverse of the transform matrix of the camera
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // projection warps the scene to give the effect of depth.
        // Far away objects appear smaller than those close.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

pub struct CameraState {
    pub camera: Camera,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraState {
    pub fn new(device: &wgpu::Device, camera: Camera) -> Self {
        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(&camera);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Camera Bind Group Layout"),
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        Self {
            camera,
            uniform,
            buffer,
            bind_group,
            bind_group_layout,

        }
    }

    pub fn update(&mut self) {
        self.uniform.update_view_proj(&self.camera);
    }
}
