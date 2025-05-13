use std::sync::Mutex;

use crate::{console_log, utils::{math::Vec3, utils::{color_to_u8, gamma_correct_color}}};


pub struct PixelBuf {
    pub width: usize,
    pub height: usize,
    pixel_rows: Vec<Mutex<Vec<Vec3>>>,
}

impl PixelBuf {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel_rows = (0..height)
            .map(|_| Mutex::new(vec![Vec3::new(0.0, 0.0, 0.0); width]))
            .collect();
        return PixelBuf {
            width,
            height,
            pixel_rows,
        }
    }
    // pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
    //     let i = y * self.width + x;
    //     self.pixels[i] = color;
    // }
    // pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
    //     let i = y * self.width + x;
    //     return self.pixels[i].clone();
    // }
    pub fn get_row_guard(&self, y: usize) -> &Mutex<Vec<Vec3>> {
        &self.pixel_rows[y]
    }
    pub fn clear_to_black(&mut self) {
        for row in self.pixel_rows.iter_mut() {
            let mut pixels = row.lock().unwrap();
            for p in pixels.iter_mut() {
                p.x = 0.0;
                p.y = 0.0;
                p.z = 0.0;
            }
        }
    }
    pub fn clear_to_white(&mut self) {
        for row in self.pixel_rows.iter_mut() {
            let mut pixels = row.lock().unwrap();
            for p in pixels.iter_mut() {
                p.x = 1.0;
                p.y = 1.0;
                p.z = 1.0;
            }
        }
    }
    // pub fn get_buf(&self) -> &Vec<Vec3> {
    //     return &self.pixels;
    // }
    // pub fn get_mut_buf(&mut self) -> &mut Vec<Vec3> {
    //     return &mut self.pixels;
    // }
    pub fn get_buf_as_u8(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.height * self.width * 4);
        for row in self.pixel_rows.iter() {
            let pixels = row.lock().unwrap();
            for p in pixels.iter() {
                let (r,g,b) = color_to_u8(p);
                buf.push(r);
                buf.push(g);
                buf.push(b);
                buf.push(255);
            }
        }
        return buf;
    }

    pub fn get_gamma_corrected_buf_as_u8(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.height * self.width * 4);
        for row in self.pixel_rows.iter() {
            let pixels = row.lock().unwrap();
            for p in pixels.iter() {
                let gamma_corrected_color = gamma_correct_color(p);
                let (r,g,b) = color_to_u8(&gamma_corrected_color);
                buf.push(r);
                buf.push(g);
                buf.push(b);
                buf.push(255);
            }
        }
        return buf;
    }
}


pub struct ZBuffer {
    pub width: usize,
    pub height: usize,
    zbuf_rows: Vec<Mutex<Vec<f32>>>,
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let zbuf_rows = (0..height)
            .map(|_| Mutex::new(vec![9999.0; width]))
            .collect();
        return ZBuffer {
            width,
            height,
            zbuf_rows,
        }
    }
    // pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) {
    //     let i = y * self.width + x;
    //     self.zbuf[i] = depth;
    // }
    // pub fn get_depth(&self, x: usize, y: usize) -> f32 {
    //     let i = y * self.width + x;
    //     return self.zbuf[i];
    // }
    pub fn get_row_guard(&self, y: usize) -> &Mutex<Vec<f32>> {
        &self.zbuf_rows[y]
    }
    pub fn clear(&mut self) {
        for row in self.zbuf_rows.iter_mut() {
            let mut zbuf = row.lock().unwrap();
            for i in 0..zbuf.len() {
                zbuf[i] = 9999.0;
            }
        }
    }
    // pub fn get_buf(&self) -> &Vec<f32> {
    //     return &self.zbuf;
    // }
}