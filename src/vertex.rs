use std::{
    cmp,
    hash::{Hash, Hasher},
};

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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let x = self.x.partial_cmp(&other.x)?;
        let y = self.y.partial_cmp(&other.y)?;
        let z = self.z.partial_cmp(&other.z)?;

        if x == y && y == z {
            Some(x)
        } else {
            None
        }
    }
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
    pub vertex: Point,
    pub normal: Normal,
}

pub(crate) struct VertexWithNormalIterator<'a, I: Iterator<Item = Point>> {
    vertices: I,
    normals: &'a [Normal],
    vertex_count: usize,
    normal_count: usize,
}

impl<'a, I: Iterator<Item = Point>> VertexWithNormalIterator<'a, I> {
    pub fn new(vertices: I, normals: &'a [Normal]) -> Self {
        Self {
            vertices,
            normals,
            vertex_count: 0,
            normal_count: 0,
        }
    }
}

impl<'a, I: Iterator<Item = Point>> Iterator for VertexWithNormalIterator<'a, I> {
    type Item = VertexWithNormal;

    fn next(&mut self) -> Option<Self::Item> {
        let v = VertexWithNormal {
            vertex: self.vertices.next()?,
            normal: self.normals[self.normal_count],
        };

        if self.vertex_count > 0 && self.vertex_count % 3 == 0 {
            self.normal_count += 1;
        }

        self.vertex_count += 1;

        Some(v)
    }
}
