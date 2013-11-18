use gl;

use core;
use geometry::Geometry;
use shader::Shader;

use cgmath;
use cgmath::ptr::*;

pub struct FrameBuffer {
    id: uint,
    width: uint,
    height: uint
}

pub struct DrawTarget {
    width: uint,
    height: uint
}


pub trait Uniforms {
    fn bind(&self, idx: i32);
}

impl Uniforms for cgmath::matrix::Mat4<f32> {
    fn bind(&self, idx: i32) {
        unsafe {
            gl::UniformMatrix4fv(idx, 1, gl::FALSE, self.ptr());
        }
    }
}

pub struct Texture {
    id: uint
}


impl core::DrawTarget for DrawTarget  {
    fn draw(&mut self, s: &Shader, g: &Geometry, uni: &[(&str, &Uniforms)], _: &[&Texture])
    {
        for uni in uni.iter() {
            let (name, u) = *uni;
            u.bind(s.uniform(name));
        }
        s.bind();
        g.draw();
    }
}

impl core::DrawSize for DrawTarget {
    fn size(&self) -> (uint, uint)
    {
        (self.width, self.height)
    }
}

impl core::DrawSize for FrameBuffer {
    fn size(&self) -> (uint, uint)
    {
        (self.width, self.height)
    }
}

impl core::FrameBuffer for FrameBuffer {
    fn viewport(&mut self,
                offset: (uint, uint), size: (uint, uint),
                f: &fn(&mut core::DrawTarget))
    {
        let (w, h) = size;
        let (x, y) = offset;
        let mut draw_target = DrawTarget {
            width: w,
            height: h
        };

        let w = w as i32;
        let h = h as i32;
        let x = x as i32;
        let y = y as i32;

        let old = &mut [0i32, 0i32, 0i32, 0i32];
        unsafe {
            do old.as_mut_buf |ptr, _| {
                gl::GetIntegerv(gl::VIEWPORT, ptr);
            }
        }

        /* set new values */
        gl::Viewport(x, y, w, h);
        gl::Scissor(x, y, w, h);
      
        let temp = &mut [0i32, 0i32, 0i32, 0i32];
        unsafe {
            do temp.as_mut_buf |ptr, _| {
                gl::GetIntegerv(gl::VIEWPORT, ptr);
            }
        }

        f(&mut draw_target as &mut core::DrawTarget);

        /* restore */
        gl::Viewport(old[0], old[1], old[2], old[3]);
        gl::Scissor(old[0], old[1], old[2], old[3]);
    }
}