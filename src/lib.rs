#![warn(missing_debug_implementations)]

use std::{
    collections::{hash_map::Entry, HashMap},
    hash::{Hash, Hasher},
};

mod ascii;
mod bbox;
mod binary;

pub use ascii::AsciiParser;
use bbox::BoundingBox;
pub use binary::BinaryParser;

#[derive(Debug, Clone, PartialEq)]
pub struct StlFile<'a> {
    header: &'a [u8],
    normals: Vec<Normal>,
    vertices: Vec<f32>,
}

impl<'a> StlFile<'a> {
    pub fn parse(buffer: &'a [u8]) -> Result<Self, ()> {
        if buffer.starts_with(b"solid") {
            Ok(AsciiParser::new(buffer)?.parse())
        } else {
            Ok(BinaryParser::new(buffer)?.parse())
        }
    }

    pub fn parse_ascii(buffer: &'a [u8]) -> Result<Self, ()> {
        Ok(AsciiParser::new(buffer)?.parse())
    }

    pub fn parse_binary(buffer: &'a [u8]) -> Result<Self, ()> {
        Ok(BinaryParser::new(buffer)?.parse())
    }

    pub fn vertex_buffer(&'a self) -> &'a [f32] {
        &self.vertices
    }

    pub fn normals(&'a self) -> &'a [Normal] {
        &self.normals
    }

    pub fn index_buffer(&self) -> IndexBuffer {
        IndexBuffer::from_buffer(self.vertices())
    }

    pub fn vertices(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.vertices.chunks_exact(3).map(|chunk| Point {
            x: chunk[0],
            y: chunk[1],
            z: chunk[2],
        })
    }

    pub fn vertices_and_normals(&'a self) -> Vec<f32> {
        self.vertices()
            .enumerate()
            .flat_map(|(idx, vertex)| {
                let normal = self.normals[(idx - idx % 3) / 3];
                vec![vertex.x, vertex.y, vertex.z, normal.i, normal.j, normal.k]
            })
            .collect()
    }

    pub fn triangles(&self) -> impl Iterator<Item = Triangle> {
        // todo
        Vec::new().into_iter()
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::init();

        for vertex in self.vertices() {
            bb.add_point(vertex);
        }

        bb
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Normal {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    normal: Normal,
    v0: Point,
    v1: Point,
    v2: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    fn normalize(&self) -> (i64, i64, i64) {
        (
            (self.x * 1024.0 * 1024.0).round() as i64,
            (self.y * 1024.0 * 1024.0).round() as i64,
            (self.z * 1024.0 * 1024.0).round() as i64,
        )
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.normalize() == other.normalize()
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.normalize().hash(state);
    }
}

impl Eq for Point {}

#[derive(Debug, Clone)]
pub struct VertexWithNormal {
    vertex: Point,
    normal: Normal,
}

struct VertexWithNormalIterator<'a, I: Iterator<Item = Point>> {
    vertices: I,
    normals: &'a [Normal],
    idx: usize,
}

impl<'a, I: Iterator<Item = Point>> Iterator for VertexWithNormalIterator<'a, I> {
    type Item = VertexWithNormal;

    fn next(&mut self) -> Option<Self::Item> {
        let v = VertexWithNormal {
            vertex: self.vertices.next()?,
            normal: self.normals[self.idx - self.idx % 3],
        };

        self.idx -= 1;

        Some(v)
    }
}

#[derive(Debug, Clone)]
pub struct IndexBuffer {
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl IndexBuffer {
    pub fn from_buffer(buffer: impl Iterator<Item = Point>) -> Self {
        let mut distinct_vertices = HashMap::new();
        let mut vertices = Vec::new();

        let mut indices = Vec::new();

        for vertex in buffer {
            let len = distinct_vertices.len();

            match distinct_vertices.entry(vertex) {
                Entry::Occupied(val) => indices.push(*val.get()),
                Entry::Vacant(ptr) => {
                    ptr.insert(len as u32);
                    indices.push(len as u32);

                    vertices.push(vertex.x);
                    vertices.push(vertex.y);
                    vertices.push(vertex.z);
                }
            }
        }

        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}
