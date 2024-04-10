use crate::vertex::Vertex;

pub trait Shape {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices(&self) -> Vec<u16>;
}

pub struct Circle {
    num_vertices: u16,
    // Radius for now is on the scale of 1 = full screen size.
    radius: f32,
}


impl Default for Circle {
    fn default() -> Self {
        Self {
            num_vertices: 3,
            radius: 0.5,
        }
    }
}

impl Circle {
    pub fn new(num_vertices: u16, radius: f32) -> Self {
        Self {
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
    fn vertices(&self) -> Vec<Vertex> {
        (0..self.num_vertices)
            .map(|i| {
                let theta = self.angle() * i as f32;
                Vertex::new(
                    [self.radius * theta.cos(), -self.radius * theta.sin(), 0.0],
                    [(1.0 + theta.cos()) / 2.0, (1.0 + theta.sin()) / 2.0, 1.0],
                )
            })
            .collect::<Vec<_>>()
    }

    fn indices(&self) -> Vec<u16> {
        (1u16..self.num_triangles() + 1)
            .into_iter()
            .flat_map(|i| vec![i + 1, i, 0])
            .collect::<Vec<_>>()
    }
}

pub struct Square {
    // Radius for now is on the scale of 1 = full screen size.
    length: f32,
}

impl Square {
    pub fn new(length: f32) -> Self {
        Self { length }
    }
}
impl Shape for Square {
    fn vertices(&self) -> Vec<Vertex> {
        // Center the square at the origin with half-length extents
        let half_length = self.length / 2.0;

        vec![
            // Top-left
            Vertex::new([-half_length, half_length, 0.0], [0.0, 1.0, 1.0]),
            // Top-right
            Vertex::new([half_length, half_length, 0.0], [1.0, 1.0, 1.0]),
            // Bottom-left
            Vertex::new([-half_length, -half_length, 0.0], [0.0, 0.0, 1.0]),
            // Bottom-right
            Vertex::new([half_length, -half_length, 0.0], [1.0, 0.0, 1.0]),
        ]
    }

    fn indices(&self) -> Vec<u16> {
        // Two triangles forming a square with counter-clockwise winding
        vec![0, 2, 1, 1, 2, 3]
    }
}


pub struct Rectangle {
    x_length: f32,
    y_length: f32,
}
