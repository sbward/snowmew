#![crate_id = "snowmew-editor"]
#![feature(macro_rules)]
#![feature(globs)]

extern crate glfw;
extern crate gl;
extern crate snowmew;
extern crate render = "snowmew-render";
extern crate loader = "snowmew-loader";
extern crate cgmath;
extern crate native;
extern crate green;
extern crate ovr = "oculus-vr";

use snowmew::core::Database;
use snowmew::camera::Camera;

use render::RenderManager;

use cgmath::transform::*;
use cgmath::vector::*;
use cgmath::rotation::*;
use cgmath::point::{Point, Point3};
use cgmath::quaternion::Quat;
use cgmath::matrix::{ToMat4};
use cgmath::angle::{ToRad, deg};

use loader::Obj;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    snowmew::start_manual_input(proc(im) {
        println!("Starting");
        let display = im.window((1280, 800))
                .expect("Could not create a display");

        let mut db = Database::new();
        let camera_loc = db.new_object(None, "camera");

        let import = Obj::load(&Path::new("assets/suzanne.obj"))
                .expect("Could not fetch suzanne");

        import.import(db.add_dir(None, "import"), &mut db);

        let scene = db.new_object(None, "scene");

        let gray = db.find("core/material/flat/white")
                .expect("Could not find gray");
        let cube = db.find("core/geometry/cube")
                .expect("Could not find cube");

        let rcube = db.new_object(Some(scene), "cube");
        db.set_draw(rcube, cube, gray);
        db.update_location(rcube,
                        Transform3D::new(1f32,
                            Rotation3::from_euler(deg(0f32).to_rad(), deg(0f32).to_rad(), deg(0f32).to_rad()),
                            Vec3::new(0 as f32, 0 as f32, 0f32)));
        
        db.update_location(camera_loc,
            Transform3D::new(1f32,
                             Rotation3::from_euler(deg(45f32).to_rad(), deg(45f32).to_rad(), deg(45f32).to_rad()),
                             Vec3::new(-10f32, -10f32, -10f32)));

        let ih = display.handle();
        let last_input = im.get(&ih);
        let (wx, wy) = last_input.screen_size();

        let mut ren = RenderManager::new(db.clone(), display, (wx, wy));
        let mut last_input = im.get(&ih);

        let (mut rot_x, mut rot_y) = (45_f64, 45_f64);
        let mut pos = Point3::new(5f32, 5f32, -5f32);

        while !last_input.should_close() {
            im.wait();
            let input_state = im.get(&ih);

            let rot: Quat<f32> =  Rotation3::from_axis_angle(&Vec3::new(0f32, 1f32, 0f32), deg(-rot_x as f32).to_rad());
            let rot = rot.mul_q(&Rotation3::from_axis_angle(&Vec3::new(1f32, 0f32, 0f32), deg(rot_y as f32).to_rad()));
            let camera = Camera::new(rot.clone(), Transform3D::new(1f32, rot, pos.to_vec()).to_mat4());
            let head_trans = Transform3D::new(1f32, rot, pos.to_vec());

            db.update_location(camera_loc, head_trans);
            ren.update(db.clone(), scene, camera_loc);
            last_input = input_state;
        }
    });
}