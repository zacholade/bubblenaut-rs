use std::fmt::format;

use wgpu::{Device, Extent3d};
use image::{self, GenericImageView};
use image::GenericImage;

struct Image {
    texture_size: wgpu::Extent3d

}

impl Image {
    pub fn new(device: Device) {
        let diffuse_bytes = include_bytes!("../assets/mainlev_build.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgb8();
        let dimensions = diffuse_image.dimensions();
    }
}