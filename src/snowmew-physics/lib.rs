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

#![crate_name = "snowmew-physics"]
#![license = "ASL2"]
#![crate_type = "lib"]
#![comment = "A collison detection manager for snowmew"]

extern crate cow;
extern crate cgmath;
extern crate collision;

extern crate snowmew  = "snowmew-core";
extern crate position = "snowmew-position";

use snowmew::common::{ObjectKey, Common};
use position::Positions;

use collision::aabb::{Aabb3};

use cgmath::Point3;
use cgmath::{Vector3};

use cow::btree::BTreeMap;

pub mod manager;

#[deriving(Clone)]
struct Collider(Aabb3<f32>);

impl std::default::Default for Collider {
    fn default() -> Collider {
        Collider(Aabb3::new(Point3::new(0f32, 0., 0.),
                            Point3::new(0f32, 0., 0.)))
    }
}

#[deriving(Clone)]
struct Velocity(Vector3<f32>);

impl std::default::Default for Velocity {
    fn default() -> Velocity {
        Velocity(Vector3::new(0f32, 0., 0.))
    }
}

#[deriving(Clone)]
pub struct PhysicsData {
    static_colliders: BTreeMap<ObjectKey, Collider>,
    colliders: BTreeMap<ObjectKey, Collider>,
    velocity: BTreeMap<ObjectKey, Velocity>,
    static_version: uint
}

impl PhysicsData {
    pub fn new() -> PhysicsData {
        PhysicsData {
            static_colliders: BTreeMap::new(),
            colliders: BTreeMap::new(),
            velocity: BTreeMap::new(),
            static_version: 0
        }
    }
}

pub trait Physics: Common + Positions {
    fn get_physics<'a>(&'a self) -> &'a PhysicsData;
    fn get_physics_mut<'a>(&'a mut self) -> &'a mut PhysicsData;

    fn add_static_collider(&mut self, key: ObjectKey, collider: Aabb3<f32>) {
        self.get_physics_mut().static_version += 1;
        self.get_physics_mut().static_colliders.insert(key, Collider(collider));   
    }

    fn get_static_collider<'a>(&'a self, key: ObjectKey) -> Option<&'a Aabb3<f32>> {
        match self.get_physics().static_colliders.find(&key) {
            Some(&Collider(ref c)) => Some(c),
            None => None
        }
    }

    fn add_collider(&mut self, key: ObjectKey, collider: Aabb3<f32>) {
        self.get_physics_mut().colliders.insert(key, Collider(collider));   
    }

    fn get_collider<'a>(&'a self, key: ObjectKey) -> Option<&'a Aabb3<f32>> {
        match self.get_physics().colliders.find(&key) {
            Some(&Collider(ref c)) => Some(c),
            None => None
        }
    }

    fn set_velocity(&mut self, key: ObjectKey, v: Vector3<f32>) {
        self.get_physics_mut().velocity.insert(key, Velocity(v));
    }

    fn get_velocity(&self, key: ObjectKey) -> Option<Vector3<f32>> {
        match self.get_physics().velocity.find(&key) {
            Some(&Velocity(ref dat)) => Some(dat.clone()),
            None => None
        }
    }
}

