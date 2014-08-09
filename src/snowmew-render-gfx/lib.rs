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

#![crate_name = "snowmew-render-gfx"]
#![crate_type = "lib"]

#![feature(phase)]
#![allow(dead_code)]

//extern crate debug;
extern crate std;
extern crate sync;
extern crate time;
extern crate libc;

extern crate opencl;
extern crate cow;
extern crate gl;
extern crate glfw;

#[phase(plugin)]
extern crate gfx_macros;
extern crate gfx;
extern crate device;
extern crate render;

extern crate snowmew  = "snowmew-core";
extern crate position = "snowmew-position";
extern crate graphics = "snowmew-graphics";
extern crate render_data = "snowmew-render-data";

use std::collections::hashmap::HashMap;

use opencl::hl::Device;
use sync::Arc;

use position::Positions;
use graphics::Graphics;
use snowmew::common::ObjectKey;
use snowmew::io::Window;

use graphics::geometry::{Geo, GeoTex, GeoNorm, GeoTexNorm, GeoTexNormTan};

use cow::join::{join_set_to_map, join_maps};
use render_data::RenderData;

static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_150: b"
    #version 150 core
    uniform mat4 proj_mat;
    uniform mat4 view_mat;
    uniform mat4 model_mat;

    in vec3 position;
    in vec2 texture;

    out vec2 o_texture;

    void main() {
        gl_Position = proj_mat * view_mat * model_mat * vec4(position, 1.0);
        o_texture = texture;
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_150: b"
    #version 150 core
    uniform vec4 ka_color;
    uniform int ka_use_texture;
    uniform sampler2D ka_texture;

    in vec2 o_texture;

    out vec4 o_Color;

    void main() {
        if (1 == ka_use_texture) {
            o_Color = texture(ka_texture, o_texture);
        } else {
            o_Color = ka_color;
        }
    }
"
};

#[shader_param]
struct Params {
    proj_mat: [[f32, ..4], ..4],
    view_mat: [[f32, ..4], ..4],
    model_mat: [[f32, ..4], ..4],
    ka_color: [f32, ..4],
    ka_use_texture: i32,
    ka_texture: gfx::shade::TextureParam,
}

struct Mesh {
    mesh: render::mesh::Mesh,
    index: render::BufferHandle
}

pub struct RenderManager {
    client: gfx::Renderer,
    frame: render::target::Frame,
    state: render::state::DrawState,
    prog: render::shade::CustomShell<_ParamsLink,Params>,
    meshes: HashMap<ObjectKey, Mesh>,
    textures: HashMap<ObjectKey, render::TextureHandle>,
    sampler: render::SamplerHandle
}

impl RenderManager {

    fn _new(server: gfx::Device<render::Token, device::GlBackEnd, Window>,
            mut client: gfx::Renderer,
            size: (i32, i32),
            _: Option<Arc<Device>>) -> RenderManager {

        glfw::make_context_current(None);
        spawn(proc() {
            let mut server = server;
            server.make_current();
            let mut start = time::precise_time_s();
            loop { 
                server.update();
                let end = time::precise_time_s();
                //println!("{:4.1}fps", 1. / (end - start));
                start = end;
            }
        });

        let (width, height) = size;
        let frame =  gfx::Frame::new(width as u16, height as u16);
        let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

        let sampler = client.create_sampler(
            gfx::tex::SamplerInfo::new(
                    gfx::tex::Bilinear, gfx::tex::Tile
            )
        );

        let prog = {
            let tinfo = gfx::tex::TextureInfo {
                width: 1,
                height: 1,
                depth: 1,
                mipmap_range: (0, 1),
                kind: gfx::tex::Texture2D,
                format: gfx::tex::RGBA8,
            };

            let img_info = tinfo.to_image_info();
            let dummy_texture = client.create_texture(tinfo);
            client.update_texture(dummy_texture, img_info, vec![0u8, 0, 0, 0]);

            let data = Params {
                proj_mat: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
                view_mat: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
                model_mat: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
                ka_use_texture: 0,
                ka_color: [1., 1., 1., 1.],
                ka_texture: (dummy_texture, Some(sampler))
            };
            let handle = client.create_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone());
            client.connect_program(handle, data).unwrap()
        };

        RenderManager {
            client: client,
            frame: frame,
            state: state,
            prog: prog,
            meshes: HashMap::new(),
            textures: HashMap::new(),
            sampler: sampler
        }
    }

    fn load_meshes<RD: RenderData>(&mut self, db: &RD) {
        for (oid, vb) in db.vertex_buffer_iter() {
            if self.meshes.find(oid).is_none() {
                let mesh = match vb.vertex {
                    Geo(ref d) => {
                        self.client.create_mesh(d.clone())
                    },
                    GeoTex(ref d) => {
                        self.client.create_mesh(d.clone())
                    },
                    GeoNorm(ref d) => {
                        self.client.create_mesh(d.clone())
                    },
                    GeoTexNorm(ref d) => {
                        self.client.create_mesh(d.clone())
                    },
                    GeoTexNormTan(ref d) => {
                        self.client.create_mesh(d.clone())
                    }
                };

                let vb: Vec<u32> = vb.index.iter().map(|&x| x as u32).collect();

                let index = self.client.create_buffer(Some(vb));

                self.meshes.insert(*oid, Mesh {
                    index: index,
                    mesh: mesh
                });
            }
        }
    }

    fn load_textures<RD: RenderData>(&mut self, db: &RD) {
        for (oid, text) in db.texture_iter() {
            if self.textures.find(oid).is_none() {
                let tinfo = gfx::tex::TextureInfo {
                    width: text.width() as u16,
                    height: text.height() as u16,
                    depth: 1 as u16,
                    mipmap_range: (0, 1),
                    kind: gfx::tex::Texture2D,
                    format: match text.depth() {
                        4 => gfx::tex::Unsigned(gfx::tex::RGBA, 8, gfx::attrib::IntNormalized),
                        3 => gfx::tex::Unsigned(gfx::tex::RGB, 8, gfx::attrib::IntNormalized),
                        _ => fail!("Unsupported color depth")
                    }
                };

                let img_info = tinfo.to_image_info();
                let texture = self.client.create_texture(tinfo);
                self.client.update_texture(texture, img_info, text.data().to_vec());
                self.textures.insert(*oid, texture);
            }
        }
    }

    fn draw<RD: RenderData>(&mut self, db: &RD, scene: ObjectKey, camera: ObjectKey) {
        let cdata = gfx::ClearData {
            color: Some(gfx::Color([0.3, 0.3, 0.3, 1.0])),
            depth: Some(1.0),
            stencil: None,
        };
        self.client.clear(cdata, self.frame);

        let camera_trans = db.position(camera);
        let camera = snowmew::camera::Camera::new(camera_trans);

        let proj = camera.projection_matrix(16. / 9.);
        self.prog.data.proj_mat =
            [[proj.x.x, proj.x.y, proj.x.z, proj.x.w],
             [proj.y.x, proj.y.y, proj.y.z, proj.y.w],
             [proj.z.x, proj.z.y, proj.z.z, proj.z.w],
             [proj.w.x, proj.w.y, proj.w.z, proj.w.w]];

        let view = camera.view_matrix();
        self.prog.data.view_mat =
            [[view.x.x, view.x.y, view.x.z, view.x.w],
             [view.y.x, view.y.y, view.y.z, view.y.w],
             [view.z.x, view.z.y, view.z.z, view.z.w],
             [view.w.x, view.w.y, view.w.z, view.w.w]];


        for (id, (draw, _)) in join_set_to_map(db.scene_iter(scene),
                                               join_maps(db.drawable_iter(),
                                                         db.location_iter())) {

            let geo = db.geometry(draw.geometry).expect("failed to find geometry");
            let mat = db.material(draw.material).expect("Could not find material");
            let vb = self.meshes.find(&geo.vb).expect("Could not get vertex buffer");

            let model = db.position(*id);

            self.prog.data.model_mat =
                [[model.x.x, model.x.y, model.x.z, model.x.w],
                 [model.y.x, model.y.y, model.y.z, model.y.w],
                 [model.z.x, model.z.y, model.z.z, model.z.w],
                 [model.w.x, model.w.y, model.w.z, model.w.w]];


            self.prog.data.ka_use_texture = match mat.map_ka() {
                Some(tid) => {
                    let &texture = self.textures.find(&tid).expect("Texture not loaded");
                    self.prog.data.ka_texture = (texture, Some(self.sampler));
                    1
                }
                None => {
                    let [r, g, b] = mat.ka();
                    self.prog.data.ka_color = [r, g, b, 1.];
                    0
                }
            };

            let _ = self.client.draw(&vb.mesh, 
                 gfx::IndexSlice(vb.index, geo.offset as u32, geo.count as u32),
                 &self.frame,
                 &self.prog,
                 &self.state
            );
        }

        self.client.end_frame();
    }
}


impl<RD: RenderData+Send> snowmew::Render<RD> for RenderManager {
    fn update(&mut self, db: RD, scene: ObjectKey, camera: ObjectKey) {
        self.load_meshes(&db);
        self.load_textures(&db);
        self.draw(&db, scene, camera);
    }
}

struct Wrap<'a>(&'a snowmew::io::IOManager);

impl<'a> device::GlProvider for Wrap<'a> {
    fn get_proc_address(&self, name: &str) -> *const ::libc::c_void {
        let Wrap(provider) = *self;
        provider.get_proc_address(name)
    }
}

impl<RD: RenderData+Send> snowmew::RenderFactory<RD, RenderManager> for RenderFactory {
    fn init(self: Box<RenderFactory>,
            io: &snowmew::IOManager,
            window: Window,
            size: (i32, i32),
            cl: Option<Arc<Device>>) -> RenderManager {

        window.make_context_current();

        let (renderer, device) = gfx::build()
            .with_context(window)
            .with_provider(Wrap(io))
            .with_queue_size(2)
            .create()
            .unwrap();

        RenderManager::_new(device, renderer, size, cl)
    }
}

pub struct RenderFactory;

impl RenderFactory {
    pub fn new() -> RenderFactory { RenderFactory }
}