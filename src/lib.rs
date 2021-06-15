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

    pub fn index_buffer_vertex_only(&self) -> IndexBuffer {
        IndexBuffer::from_buffer(self.vertex_and_normal_iterator(), push_vertex_only)
    }

    pub fn index_buffer_vertex_and_normal(&self) -> IndexBuffer {
        IndexBuffer::from_buffer(self.vertex_and_normal_iterator(), push_vertex_and_normal)
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

    pub fn vertex_and_normal_iterator(&'a self) -> impl Iterator<Item = VertexWithNormal> + 'a {
        VertexWithNormalIterator {
            vertices: self.vertices(),
            normals: self.normals(),
            idx: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Normal {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl Normal {
    fn normalize(&self) -> (i64, i64, i64) {
        (
            (self.i * 1024.0 * 1024.0).round() as i64,
            (self.j * 1024.0 * 1024.0).round() as i64,
            (self.k * 1024.0 * 1024.0).round() as i64,
        )
    }
}

impl PartialEq for Normal {
    fn eq(&self, other: &Self) -> bool {
        self.normalize() == other.normalize()
    }
}

impl Hash for Normal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.normalize().hash(state);
    }
}

impl Eq for Normal {}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn push_vertex_only(v: VertexWithNormal, vertices: &mut Vec<f32>) {
    vertices.push(v.vertex.x);
    vertices.push(v.vertex.y);
    vertices.push(v.vertex.z);
}

fn push_vertex_and_normal(v: VertexWithNormal, vertices: &mut Vec<f32>) {
    vertices.push(v.vertex.x);
    vertices.push(v.vertex.y);
    vertices.push(v.vertex.z);

    vertices.push(v.normal.i);
    vertices.push(v.normal.j);
    vertices.push(v.normal.k);
}

impl IndexBuffer {
    fn from_buffer(
        buffer: impl Iterator<Item = VertexWithNormal>,
        push: fn(VertexWithNormal, &mut Vec<f32>),
    ) -> Self {
        let mut distinct_vertices = HashMap::new();
        let mut vertices = Vec::new();

        let mut indices = Vec::new();

        for vertex_with_normal in buffer {
            let len = distinct_vertices.len();

            match distinct_vertices.entry(vertex_with_normal) {
                Entry::Occupied(val) => indices.push(*val.get()),
                Entry::Vacant(ptr) => {
                    ptr.insert(len as u32);
                    indices.push(len as u32);

                    push(vertex_with_normal, &mut vertices);
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
