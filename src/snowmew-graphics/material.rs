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

use std::default::Default;
use rustc_serialize::{Encodable, Encoder, Decoder};

use snowmew::Entity;
use super::geometry::F32v3;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Copy)]
pub struct Material {
    ka: F32v3,
    kd: F32v3,
    ks: F32v3,
    ke: F32v3,
    tf: F32v3,
    ns: f32,
    ni: f32,
    tr: f32,
    d: f32,
    illum: i32,

    map_ka:   Option<Entity>,
    map_kd:   Option<Entity>,
    map_ks:   Option<Entity>,
    map_ke:   Option<Entity>,
    map_ns:   Option<Entity>,
    map_d:    Option<Entity>,
    map_bump: Option<Entity>,
    map_refl: Option<Entity>,
}

impl Clone for Material {
    fn clone(&self) -> Material {
        Material {
            ka: self.ka,
            kd: self.kd,
            ks: self.ks,
            ke: self.ke,
            tf: self.tf,
            ns: self.ns,
            ni: self.ni,
            tr: self.tr,
            d: self.d,
            illum: self.illum,
            map_ka: self.map_ka,
            map_kd: self.map_kd,
            map_ks: self.map_ks,
            map_ke: self.map_ke,
            map_ns: self.map_ns,
            map_d: self.map_d,
            map_bump: self.map_bump,
            map_refl: self.map_refl
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::new()
    }
}

impl Material {
    pub fn new() -> Material {
        Material {
            ka: F32v3([0.; 3]),
            kd: F32v3([0.; 3]),
            ks: F32v3([0.; 3]),
            ke: F32v3([0.; 3]),
            tf: F32v3([0.; 3]),
            ns: 0.,
            ni: 0.,
            tr: 0.,
            d: 0.,
            illum: 2,
            map_ka:   None,
            map_kd:   None,
            map_ks:   None,
            map_ke:   None,
            map_ns:   None,
            map_d:    None,
            map_bump: None,
            map_refl: None,
        }
    }

    pub fn simple(color: [f32; 3]) -> Material {
        let mut mat = Material::new();
        mat.ka = F32v3(color);
        mat.kd = F32v3(color);
        mat.ks = F32v3(color);
        mat
    }

    pub fn ka(&self) -> [f32; 3] {self.ka.0}
    pub fn set_ka(&mut self, c: [f32; 3]) {self.ka = F32v3(c);}

    pub fn kd(&self) -> [f32; 3] {self.kd.0}
    pub fn set_kd(&mut self, c: [f32; 3]) {self.kd = F32v3(c);}

    pub fn ks(&self) -> [f32; 3] {self.ks.0}
    pub fn set_ks(&mut self, c: [f32; 3]) {self.ks = F32v3(c);}

    pub fn ke(&self) -> [f32; 3] {self.ks.0}
    pub fn set_ke(&mut self, c: [f32; 3]) {self.ke = F32v3(c);}

    pub fn tf(&self) -> [f32; 3] {self.tf.0}
    pub fn set_tf(&mut self, c: [f32; 3]) {self.tf = F32v3(c);}

    pub fn map_ka(&self) -> Option<Entity> {self.map_ka}
    pub fn set_map_ka(&mut self, oid: Entity) {self.map_ka = Some(oid);}

    pub fn map_kd(&self) -> Option<Entity> {self.map_kd}
    pub fn set_map_kd(&mut self, oid: Entity) {self.map_kd = Some(oid);}

    pub fn map_ks(&self) -> Option<Entity> {self.map_ks}
    pub fn set_map_ks(&mut self, oid: Entity) {self.map_ks = Some(oid);}

    pub fn map_ke(&self) -> Option<Entity> {self.map_ke}
    pub fn set_map_ke(&mut self, oid: Entity) {self.map_ke = Some(oid);}

    pub fn map_ns(&self) -> Option<Entity> {self.map_ns}
    pub fn set_map_ns(&mut self, oid: Entity) {self.map_ns = Some(oid);}

    pub fn map_d(&self) -> Option<Entity> {self.map_d}
    pub fn set_map_d(&mut self, oid: Entity) {self.map_d = Some(oid);}

    pub fn map_bump(&self) -> Option<Entity> {self.map_bump}
    pub fn set_map_bump(&mut self, oid: Entity) {self.map_bump = Some(oid);}

    pub fn map_refl(&self) -> Option<Entity> {self.map_refl}
    pub fn set_map_refl(&mut self, oid: Entity) {self.map_refl = Some(oid);}

    pub fn ns(&self) -> f32 {self.ns}
    pub fn set_ns(&mut self, v: f32) {self.ns = v}

    pub fn ni(&self) -> f32 {self.ni}
    pub fn set_ni(&mut self, v: f32) {self.ni = v}
}
