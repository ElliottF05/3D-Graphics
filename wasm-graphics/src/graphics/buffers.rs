use crate::utils::{math::Vec3, utils::color_to_u8};

pub struct PixelBuf {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl PixelBuf {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = vec![0; width * height * 4];
        for i in (0..pixels.len()).step_by(4) {
            pixels[i+3] = 255;
        }
        return PixelBuf {
            width,
            height,
            pixels,
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        let i = (y * self.width + x) * 4;
        let (r,g,b) = color_to_u8(color);
        self.pixels[i] = r;
        self.pixels[i+1] = g;
        self.pixels[i+2] = b;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        let i = (y * self.width + x) * 4;
        return Vec3::new(
            self.pixels[i] as f32 / 255.0,
            self.pixels[i+1] as f32 / 255.0,
            self.pixels[i+2] as f32 / 255.0,
        );
    }
    pub fn clear(&mut self) {
        for i in (0..self.pixels.len()).step_by(4) {
            self.pixels[i] = 0;
            self.pixels[i+1] = 0;
            self.pixels[i+2] = 0;
        }
    }
    pub fn get_buf(&self) -> &Vec<u8> {
        return &self.pixels;
    }
}


pub struct ZBuffer {
    pub width: usize,
    pub height: usize,
    pub zbuf: Vec<f32>,
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let zbuf = vec![9999.0; width * height];
        return ZBuffer {
            width,
            height,
            zbuf,
        }
    }
    pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) {
        let i = y * self.width + x;
        self.zbuf[i] = depth;
    }
    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        let i = y * self.width + x;
        return self.zbuf[i];
    }
    pub fn clear(&mut self) {
        for i in 0..self.zbuf.len() {
            self.zbuf[i] = 9999.0;
        }
    }
    pub fn get_buf(&self) -> &Vec<f32> {
        return &self.zbuf;
    }
}