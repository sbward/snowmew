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

#![crate_name = "gears"]
#![feature(macro_rules)]
#![feature(globs)]

extern crate glfw;
extern crate gl;
extern crate "snowmew-core" as snowmew;
extern crate "snowmew-render-mux" as render;
extern crate "snowmew-loader" as loader;
extern crate "snowmew-position" as position;
extern crate "snowmew-graphics" as graphics;
extern crate cgmath;
extern crate native;
extern crate green;
extern crate opencl;
extern crate sync;
extern crate serialize;
extern crate "snowmew-render-data" as render_data;

use cgmath::*;

use position::Positions;
use graphics::Graphics;
use graphics::light;
use render_data::Renderable;

use render::RenderFactory;
use loader::Obj;
use snowmew::input_integrator::{input_integrator, InputIntegratorState};
use snowmew::common::Common;
use snowmew::game::Game;
use snowmew::debugger::debugger;

use gamedata::GameData;

mod gamedata;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let sc = snowmew::SnowmewConfig::new();

    let mut gd = GameData::new();
    let loader = Obj::load(&Path::new("assets/rust_logo.obj")).ok().expect("Failed to load OBJ");
    let import = gd.new_object(None, "import");
    loader.import(import, &mut gd);

    let scene = gd.new_scene("scene");
    let logo = gd.find("import/objects/rust_logo").expect("geometry not found from import");
    let logo_draw = gd.get_draw(logo).expect("Could not get draw binding");

    let parent = gd.add_dir(Some(scene), "gears");

    let scene_logos = vec!((gd.new_object(Some(parent), "logo0"), "core/material/flat/red"),
                           (gd.new_object(Some(parent), "logo1"), "core/material/flat/blue"),
                           (gd.new_object(Some(parent), "logo2"), "core/material/flat/green"));

    for (idx, &(logo, material)) in scene_logos.iter().enumerate() {
        println!("idx={}, logo={}", idx, logo);
        let mat = gd.find(material).expect("material not found");
        gd.set_draw(logo, logo_draw.geometry, mat);
        gd.set_scale(logo, 0.136);
        gd.set_displacement(logo, Vector3::new(idx as f32, 0f32, 0f32));
        gd.set_rotation(logo, Rotation3::from_euler(rad(0f32),
                                                    deg(90f32).to_rad(),
                                                    deg(90f32).to_rad()));
    }

    let camera_loc = gd.new_object(None, "camera");

    gd.update_location(camera_loc, Decomposed{scale: 1f32,
                                              rot:   Rotation::identity(),
                                              disp:  Vector3::new(1f32, 0f32, 1.5f32)});

    let sun = light::Directional::new(Vector3::new(0.5f32, 1., 0.5),
                                      Vector3::new(1f32, 1., 1.), 0.25);

    gd.new_light(scene, "sun", light::DirectionalLight(sun));
    gd.set_scene(scene);
    gd.set_camera(camera_loc);

    let (game, gd) = input_integrator(Gears, gd);
    let (game, gd) = debugger(game, gd);
    sc.start(box RenderFactory::new(), game, gd);
}

struct Gears;

impl Game<GameData, InputIntegratorState> for Gears {
    fn step(&mut self, state: InputIntegratorState, gd: GameData) -> GameData {
        let mut next = gd.clone();
        let gears_dir = gd.find("scene/gears").unwrap();

        for (idx, (_, logo)) in gd.walk_dir(gears_dir).enumerate() {
            let t = state.time() as f32 * 10.;
            let this_gear_rot = if idx % 2 == 0 { t } else { 5.625 - t };
            next.set_rotation(logo, Rotation3::from_euler(deg(0f32).to_rad(),
                                                          deg(this_gear_rot).to_rad(),
                                                          deg(90f32).to_rad()));
        }

        next
    }
}
