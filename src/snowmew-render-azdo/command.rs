
//   Copyright 2014 Colin Sherratt
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::ptr;
use std::mem;
use std::slice;

use cow::join::{join_set_to_map};


use libc::c_void;

use gl;
use gl::types::{GLsizei, GLuint};

use cgmath::Matrix4;
use cgmath::Array2;
use cgmath::{Vector, EuclideanVector};

use config::Config;
use graphics::Graphics;
use snowmew::common::Entity;


use db::GlState;

#[repr(packed)]
struct DrawElementsIndirectCommand {
    pub count: GLuint,
    pub instrance_count: GLuint,
    pub first_index: GLuint,
    pub base_vertex: GLuint,
    pub base_instance: GLuint
}

pub struct CommandBufferIndirect {
    command: GLuint,
    ptr: *mut DrawElementsIndirectCommand,
    size: uint,
    batches: Vec<Batch>
}

#[deriving(Clone)]
pub struct Batch {
    vbo: Entity,
    offset: uint,
    count: uint
}

impl Batch {
    pub fn vbo(&self) -> Entity {self.vbo}

    pub fn offset(&self) -> *const c_void {
        assert!(mem::size_of::<DrawElementsIndirectCommand>() == 20);
        (self.offset * mem::size_of::<DrawElementsIndirectCommand>()) as *const c_void
    }

    pub fn drawcount(&self) -> GLsizei {
        self.count as GLsizei
    }

    pub fn stride(&self) -> GLsizei {
        assert!(mem::size_of::<DrawElementsIndirectCommand>() == 20);
        mem::size_of::<DrawElementsIndirectCommand>() as GLsizei
    }

    pub fn offset_int(&self) -> uint {self.offset}
}


impl CommandBufferIndirect {
    pub fn new(cfg: &Config) -> CommandBufferIndirect {
        let cb = &mut [0];
        unsafe {
            gl::GenBuffers(1, cb.unsafe_mut(0));
            gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, cb[0]);
            gl::BufferData(gl::DRAW_INDIRECT_BUFFER,
                           (mem::size_of::<DrawElementsIndirectCommand>() *
                           cfg.max_size()) as i64,
                           ptr::null(),
                           gl::DYNAMIC_DRAW);
            gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, 0);
        }

        CommandBufferIndirect {
            command: cb[0],
            ptr: ptr::null_mut(),
            size: cfg.max_size(),
            batches: Vec::new()
        }
    }

    pub fn map(&mut self) { unsafe {
        gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, self.command);
        self.ptr = gl::MapBufferRange(
            gl::DRAW_INDIRECT_BUFFER, 0,
            (mem::size_of::<DrawElementsIndirectCommand>() *
            self.size) as i64,
            gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT
        ) as *mut DrawElementsIndirectCommand;
        gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, 0);
    }}

    pub fn unmap(&mut self) { unsafe {
        self.ptr = ptr::null_mut();
        gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, self.command);
        gl::UnmapBuffer(gl::DRAW_INDIRECT_BUFFER);
        gl::BindBuffer(gl::DRAW_INDIRECT_BUFFER, 0);
    }}

    pub fn build<GD: Graphics>(&mut self, db: &GD, scene: Entity, instanced_is_enabled: bool) {
        let mut batch = Batch {
            vbo: 0,
            offset: 0,
            count: 0
        };

        let mut idx = -1;
        let mut last_geo = None;
        let mut command = DrawElementsIndirectCommand {
            count: 0,
            instrance_count: 1,
            first_index: 0,
            base_vertex: 0,
            base_instance: 0
        };

        unsafe {
            self.batches.truncate(0);
            let b = slice::from_raw_mut_buf(&self.ptr, self.size);
            for (count, (_, draw)) in join_set_to_map(db.scene_iter(scene), db.drawable_iter()).enumerate() {
                if idx == -1 {
                    let draw_geo = db.geometry(draw.geometry).expect("geometry not found");
                    last_geo = Some(draw.geometry);
                    command = DrawElementsIndirectCommand {
                        count: draw_geo.count as GLuint,
                        instrance_count: 1,
                        first_index: draw_geo.offset as GLuint,
                        base_vertex: 0,
                        base_instance: count as GLuint
                    };

                    batch.vbo = draw_geo.vb;
                    batch.count = 1;

                    idx = 0;
                } else if last_geo == Some(draw.geometry) && instanced_is_enabled {
                    command.instrance_count += 1;
                } else {
                    let draw_geo = db.geometry(draw.geometry).expect("geometry not found");
                    last_geo = Some(draw.geometry);

                    b[idx] = command;
                    idx += 1;

                    command = DrawElementsIndirectCommand {
                        count: draw_geo.count as GLuint,
                        instrance_count: 1,
                        first_index: draw_geo.offset as GLuint,
                        base_vertex: 0,
                        base_instance: count as GLuint
                    };

                    if batch.vbo == 0 {
                        batch.vbo = draw_geo.vb;
                        batch.offset = idx;
                        batch.count = 1;
                    } else if batch.vbo == draw_geo.vb {
                        batch.count += 1;
                    } else {
                        self.batches.push(batch.clone());
                        batch.vbo = draw_geo.vb;
                        batch.offset = idx;
                        batch.count = 1;
                    }

                }
            }
            b[idx] = command;
        }
        self.batches.push(batch)
    }

    pub fn cull(&self, draw: GLuint, matrix: GLuint, dat: &GlState, mat: &Matrix4<f32>) {
        let to_plane = |x, scale| {
            let plane = mat.row(x).mul_s(scale).add_v(&mat.row(3));
            plane.normalize()
        };

        let planes = &[
            to_plane(0,  1.),
            to_plane(0, -1.),
            to_plane(1,  1.),
            to_plane(1, -1.),
            to_plane(2,  1.),
            to_plane(2, -1.)
        ];

        let shader = dat.compute_cull.as_ref().expect("Could not get cull");

        let size = self.batches.iter().fold(0, |a, b| a + b.count);

        let x = 256i;
        let y = size / 256 + 1;

        shader.bind();
        unsafe {
            gl::Uniform4fv(shader.uniform("plane"), 6, &planes[0].x);
            gl::Uniform1i(shader.uniform("max_id"), size as i32);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, draw);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, matrix);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 2, self.command);
            gl::DispatchCompute(x as u32, y as u32, 1);
            gl::MemoryBarrier(gl::COMMAND_BARRIER_BIT);
        }
    }

    pub fn batches<'a>(&'a self) -> &'a [Batch] {
        self.batches.slice(0, self.batches.len())
    }

    pub fn id(&self) -> GLuint {
        self.command
    }
}

pub struct CommandBufferEmulated {
    commands: Vec<DrawElementsIndirectCommand>,
    batches: Vec<Batch>
}

impl CommandBufferEmulated {
    pub fn new(_: &Config) -> CommandBufferEmulated {
        CommandBufferEmulated {
            commands: Vec::new(),
            batches: Vec::new()
        }
    }

    pub fn map(&mut self) {}
    pub fn unmap(&mut self) {}

    pub fn build<GD: Graphics>(&mut self, db: &GD, scene: Entity, _: bool) {
        let mut batch = Batch {
            vbo: 0,
            offset: 0,
            count: 0
        };

        self.batches.truncate(0);
        self.commands.truncate(0);
        for (count, (_, draw)) in join_set_to_map(db.scene_iter(scene), db.drawable_iter()).enumerate() {
            let draw_geo = db.geometry(draw.geometry).expect("geometry not found");

            self.commands.push(DrawElementsIndirectCommand {
                count: draw_geo.count as GLuint,
                instrance_count: 1,
                first_index: draw_geo.offset as GLuint,
                base_vertex: 0,
                base_instance: count as GLuint
            });

            if batch.vbo == 0 {
                batch.vbo = draw_geo.vb;
                batch.offset = count;
                batch.count = 1;
            } else if batch.vbo == draw_geo.vb {
                batch.count += 1;
            } else {
                self.batches.push(batch.clone());
                batch.vbo = draw_geo.vb;
                batch.offset = count;
                batch.count = 1;
            }
        }
      
        self.batches.push(batch)
    }

    pub fn batches<'a>(&'a self) -> &'a [Batch] {
        self.batches.slice(0, self.batches.len())
    }

    pub fn commands<'a>(&'a self) -> &'a [DrawElementsIndirectCommand] {
        self.commands.slice(0, self.commands.len())
    }
}