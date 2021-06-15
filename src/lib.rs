#![warn(missing_debug_implementations)]

use std::collections::{hash_map::Entry, HashMap};

mod ascii;
mod bbox;
mod binary;
mod vertex;

pub use ascii::AsciiParser;
use bbox::BoundingBox;
pub use binary::BinaryParser;
use vertex::VertexWithNormalIterator;
pub use vertex::{Normal, Point, Triangle, VertexWithNormal};

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
        VertexWithNormalIterator::new(self.vertices(), self.normals())
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
