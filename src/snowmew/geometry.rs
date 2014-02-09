
use std::vec;

use cgmath::vector::{Vec2, Vec3};

use core::object_key;

#[deriving(Clone)]
pub enum Primative {
    Point,
    Line,
    Triangle,
    TriangleAdjacency
}

#[deriving(Clone)]
pub struct VertexGeo
{
    position: Vec3<f32>
}

#[deriving(Clone)]
pub struct VertexGeoTex
{
    position: Vec3<f32>,
    texture: Vec2<f32>
}

#[deriving(Clone)]
pub struct VertexGetTexNorm
{
    position: Vec3<f32>,
    texture: Vec2<f32>,
    normal: Vec3<f32>
}

#[deriving(Clone)]
pub enum Vertex {
    Empty,
    Geo(~[VertexGeo]),
    GeoTex(~[VertexGeoTex]),
    GeoTexNorm(~[VertexGetTexNorm])
}

impl Default for Vertex
{
    fn default() -> Vertex
    {
        return Empty
    }
}

#[deriving(Clone, Default)]
pub struct VertexBuffer {
    vertex: Vertex,
    index: ~[u32]
}


#[deriving(Clone, Default)]
pub struct Geometry {
    vb: object_key,
    count: uint, // number of index elements
    offset: uint, // offset into the index buffer
    prim: Primative
}

impl Default for Primative {
    fn default() -> Primative {Point}
}

fn find_trig<IDX: Eq+Clone>(index: &[IDX], my_idx: uint, a: IDX, b: IDX) -> IDX
{
    let my_idx = my_idx as int;
    for i in range(0, (index.len()/3) as int) {
        if i != my_idx {
            /* look for candidate */
            let mut found_a = -1;
            let mut found_b = -1;
            for j in range(0, 3) {
                if a == index[i*3+j] {
                    found_a = j;
                }
                if b == index[i*3+j] {
                    found_b = j;
                }
            }

            /* found a candidate */
            if found_a != -1 && found_b != -1  {
                for j in range(0, 3) {
                    if j != found_a && j != found_b {
                        return index[i*3+j].clone();
                    }
                }
            }
        }
    }
    fail!("Did not find vertex!");
}


pub fn to_triangles_adjacency<IDX: Eq+Clone>(index: &[IDX]) -> ~[IDX]
{
    vec::build(Some(index.len() * 2), |emit| {
        for i in range(0, index.len()/3) {
            let a = &index[i*3];
            let b = &index[i*3+1];
            let c = &index[i*3+2];

            emit(a.clone());
            emit(find_trig(index, i, a.clone(), b.clone()).clone());
            emit(b.clone());
            emit(find_trig(index, i, b.clone(), c.clone()).clone());
            emit(c.clone());
            emit(find_trig(index, i, c.clone(), a.clone()).clone());
        }
    })
}

impl Geometry {
    pub fn triangles(vb: object_key, offset: uint, count: uint) -> Geometry
    {
        Geometry {
            vb: vb,
            count: count,
            offset: offset,
            prim: Triangle
        }
    }

    pub fn triangles_adjacency(vb: object_key, offset: uint, count: uint) -> Geometry
    {
        Geometry {
            vb: vb,
            count: count,
            offset: offset,
            prim: TriangleAdjacency
        }
    }

    pub fn lines(vb: object_key, offset: uint, count: uint) -> Geometry
    {
        Geometry {
            vb: vb,
            count: count,
            offset: offset,
            prim: Line
        }
    }

    pub fn points(vb: object_key, offset: uint, count: uint) -> Geometry
    {
        Geometry {
            vb: vb,
            count: count,
            offset: offset,
            prim: Point
        }
    }
}

impl VertexBuffer {
    pub fn new_position(vert: ~[VertexGeo], idx: ~[u32]) -> VertexBuffer {
        VertexBuffer {
            vertex: Geo(vert),
            index: idx
        }
    }

    pub fn new_position_texture(vert: ~[VertexGeoTex], idx: ~[u32]) -> VertexBuffer {
        VertexBuffer {
            vertex: GeoTex(vert),
            index: idx
        }
    }

    pub fn new_position_texture_normal(vert: ~[VertexGetTexNorm], idx: ~[u32]) -> VertexBuffer {
        VertexBuffer {
            vertex: GeoTexNorm(vert),
            index: idx
        }
    }
}