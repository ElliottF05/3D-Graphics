use crate::utils::{math::Vec3, utils::color_to_u8};

#[derive(Clone)]
pub struct PixelBuf {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec3>,
}

impl PixelBuf {
    pub fn new(width: usize, height: usize) -> Self {
        return PixelBuf {
            width,
            height,
            pixels: vec![Vec3::new(0.0, 0.0, 0.0); width * height],
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        let i = y * self.width + x;
        self.pixels[i] = color;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        let i = y * self.width + x;
        return self.pixels[i].clone();
    }
    pub fn clear_to_black(&mut self) {
        for p in self.pixels.iter_mut() {
            p.x = 0.0;
            p.y = 0.0;
            p.z = 0.0;
        }
    }
    pub fn clear_to_white(&mut self) {
        for p in self.pixels.iter_mut() {
            p.x = 1.0;
            p.y = 1.0;
            p.z = 1.0;
        }
    }
    pub fn get_buf(&self) -> &Vec<Vec3> {
        return &self.pixels;
    }
    pub fn get_mut_buf(&mut self) -> &mut Vec<Vec3> {
        return &mut self.pixels;
    }
    pub fn get_buf_as_u8(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.height * self.width * 4);
        for p in self.pixels.iter() {
            let (r,g,b) = color_to_u8(p);
            buf.push(r);
            buf.push(g);
            buf.push(b);
            buf.push(255);
        }
        return buf;
    }
}

#[derive(Clone)]
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