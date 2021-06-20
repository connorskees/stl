#![warn(missing_debug_implementations)]

use std::{
    collections::{hash_map::Entry, HashMap},
    io::{Result as IoResult, Write},
};

mod ascii;
mod bbox;
mod binary;
mod vertex;

use ascii::AsciiParser;
pub use bbox::BoundingBox;
use binary::BinaryParser;
pub use vertex::{Normal, Point, Triangle, VertexWithNormal};
use vertex::{TriangleIterator, VertexWithNormalIterator};

#[derive(Debug, Clone, PartialEq)]
pub struct StlFile {
    normals: Vec<Normal>,
    vertices: Vec<f32>,
}

impl StlFile {
    pub fn parse(buffer: &[u8]) -> Result<Self, ()> {
        if buffer.starts_with(b"solid") {
            Ok(AsciiParser::new(buffer)?.parse())
        } else {
            Ok(BinaryParser::new(buffer)?.parse())
        }
    }

    pub fn parse_ascii(buffer: &[u8]) -> Result<Self, ()> {
        Ok(AsciiParser::new(buffer)?.parse())
    }

    pub fn parse_binary(buffer: &[u8]) -> Result<Self, ()> {
        Ok(BinaryParser::new(buffer)?.parse())
    }

    pub fn write_binary(&self, buffer: &mut dyn Write) -> IoResult<()> {
        buffer.write_all(&[0; 80])?;
        buffer.write_all(&self.vertex_count().to_le_bytes())?;

        fn write_vec3(x: f32, y: f32, z: f32, buffer: &mut dyn Write) -> IoResult<()> {
            buffer.write_all(&x.to_le_bytes())?;
            buffer.write_all(&y.to_le_bytes())?;
            buffer.write_all(&z.to_le_bytes())?;

            Ok(())
        }

        for triangle in self.triangles() {
            write_vec3(
                triangle.normal.i,
                triangle.normal.j,
                triangle.normal.k,
                buffer,
            )?;
            write_vec3(triangle.v0.x, triangle.v0.y, triangle.v0.z, buffer)?;
            write_vec3(triangle.v1.x, triangle.v1.y, triangle.v1.z, buffer)?;
            write_vec3(triangle.v2.x, triangle.v2.y, triangle.v2.z, buffer)?;

            buffer.write_all(&[0; 2])?;
        }

        Ok(())
    }

    pub fn vertex_buffer(&self) -> &[f32] {
        &self.vertices
    }

    fn vertex_count(&self) -> u32 {
        debug_assert_eq!(self.vertices.len() % 3, 0);
        (self.vertices.len() / 3) as u32
    }

    pub fn normals<'a>(&'a self) -> &'a [Normal] {
        &self.normals
    }

    pub fn index_buffer_vertex_only(&self) -> IndexBuffer {
        IndexBuffer::from_buffer(self.vertex_and_normal_iterator(), push_vertex_only)
    }

    pub fn index_buffer_vertex_and_normal(&self) -> IndexBuffer {
        IndexBuffer::from_buffer(self.vertex_and_normal_iterator(), push_vertex_and_normal)
    }

    pub fn vertices<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.vertices.chunks_exact(3).map(|chunk| Point {
            x: chunk[0],
            y: chunk[1],
            z: chunk[2],
        })
    }

    pub fn vertices_and_normals(&self) -> Vec<f32> {
        self.vertex_and_normal_iterator()
            .flat_map(|VertexWithNormal { vertex, normal }| {
                vec![vertex.x, vertex.y, vertex.z, normal.i, normal.j, normal.k]
            })
            .collect()
    }

    pub fn triangles<'a>(&'a self) -> impl Iterator<Item = Triangle> + 'a {
        TriangleIterator::new(self.vertices(), self.normals())
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::init();

        for vertex in self.vertices() {
            bb.add_point(vertex);
        }

        bb
    }

    pub fn vertex_and_normal_iterator<'a>(&'a self) -> impl Iterator<Item = VertexWithNormal> + 'a {
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
