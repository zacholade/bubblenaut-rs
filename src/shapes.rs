use crate::vertex::{ColoredVertex, TexturedVertex};

pub trait Shape {
    fn tex_vertices(&self, tex_coords: [f32; 2]) -> Vec<TexturedVertex>;
    fn col_vertices(&self, color: [f32; 4]) -> Vec<ColoredVertex>;
    fn indices(&self) -> Vec<u16>;
}

pub struct Circle {
    position: [f32; 2],
    num_vertices: u16,
    // Radius for now is on the scale of 1 = full screen size.
    radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            num_vertices: 20,
            radius: 0.5,
        }
    }
}

impl Circle {
    pub fn new(position: [f32; 2], num_vertices: u16, radius: f32) -> Self {
        Self {
            position,
            num_vertices,
            radius,
        }
    }

    pub fn angle(&self) -> f32 {
        std::f32::consts::PI * 2.0 / self.num_vertices as f32
    }

    pub fn num_triangles(&self) -> u16 {
        self.num_vertices - 2
    }
}
impl Shape for Circle {
    fn col_vertices(&self, color: [f32; 4]) -> Vec<ColoredVertex> {
        let angle_step = 2.0 * std::f32::consts::PI / self.num_vertices as f32;
        (0..self.num_vertices)
            .map(|i| {
                let theta = - angle_step * i as f32;
                let position = [
                    self.position[0] + self.radius * theta.cos(),
                    self.position[1] + self.radius * theta.sin(),
                    0.0,
                ];
                ColoredVertex::new(position, color)
            })
            .collect()
    }

    fn tex_vertices(&self, tex_coords: [f32; 2]) -> Vec<TexturedVertex> {
        let angle_step = 2.0 * std::f32::consts::PI / self.num_vertices as f32;
        (0..self.num_vertices)
            .map(|i| {
                let theta = - angle_step * i as f32;
                let position = [
                    self.position[0] + (self.radius * theta.cos()),
                    self.position[1] + (self.radius * theta.sin()),
                    0.0,
                ];
                TexturedVertex::new(position, tex_coords)
            })
            .collect()
    }

    fn indices(&self) -> Vec<u16> {
        (1u16..self.num_triangles() + 1)
            .into_iter()
            .flat_map(|i| vec![i + 1, i, 0])
            .collect::<Vec<_>>()
    }
}

pub struct Square {
    position: [f32; 2],
    // Radius for now is on the scale of 1 = full screen size.
    length: f32,
}

impl Square {
    pub fn new(position: [f32; 2], length: f32) -> Self {
        Self { position, length }
    }
}
impl Shape for Square {
    fn col_vertices(&self, color: [f32; 4]) -> Vec<ColoredVertex> {
        let half_length = self.length / 2.0;
        vec![
            // Top-left
            ColoredVertex::new([-half_length, half_length, 0.0], color),
            // Top-right
            ColoredVertex::new([half_length, half_length, 0.0], color),
            // Bottom-left
            ColoredVertex::new([-half_length, -half_length, 0.0], color),
            // Bottom-right
            ColoredVertex::new([half_length, -half_length, 0.0], color),
        ]
    }
    fn tex_vertices(&self, tex_coords: [f32; 2]) -> Vec<TexturedVertex> {
        let half_length = self.length / 2.0;
        vec![
            // Top-left
            TexturedVertex::new([-half_length, half_length, 0.0], tex_coords),
            // Top-right
            TexturedVertex::new([half_length, half_length, 0.0], tex_coords),
            // Bottom-left
            TexturedVertex::new([-half_length, -half_length, 0.0], tex_coords),
            // Bottom-right
            TexturedVertex::new([half_length, -half_length, 0.0], tex_coords),
        ]
    }

    fn indices(&self) -> Vec<u16> {
        // Two triangles forming a square with counter-clockwise winding
        vec![0, 2, 1, 1, 2, 3]
    }
}
