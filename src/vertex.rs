use std::{
    cmp,
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Normal {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl Normal {
    pub const fn zero() -> Self {
        Self {
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub normal: Normal,
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
}

pub struct Intersection {}

struct LineSegment {
    start: Point,
    end: Point,
}

impl Triangle {
    pub fn intersect_triangle(&self, other: &Self) -> Option<Intersection> {
        let n2 = (other.v1 - other.v0) * (other.v2 - other.v0);
        let d2 = -n2 * other.v0;

        let dv1_0 = n2 * self.v0 + d2;
        let dv1_1 = n2 * self.v1 + d2;
        let dv1_2 = n2 * self.v2 + d2;

        let same_sign = dv1_0.sign() == dv1_1.sign() && dv1_1.sign() == dv1_2.sign();

        // if

        todo!()

        // let pi2 = n2 * other.v0 + d2
    }

    fn edges(&self) -> [LineSegment; 3] {
        [
            LineSegment {
                start: self.v0,
                end: self.v1,
            },
            LineSegment {
                start: self.v1,
                end: self.v2,
            },
            LineSegment {
                start: self.v2,
                end: self.v0,
            },
        ]
    }
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
    pub(crate) fn normalize(&self) -> (i64, i64, i64) {
        (
            (self.x * 1024.0 * 1024.0).round() as i64,
            (self.y * 1024.0 * 1024.0).round() as i64,
            (self.z * 1024.0 * 1024.0).round() as i64,
        )
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn sign(&self) -> (f32, f32, f32) {
        (self.x.signum(), self.y.signum(), self.z.signum())
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

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
            vertex_count: 1,
            normal_count: 0,
        }
    }
}

impl<'a, I: Iterator<Item = Point>> Iterator for VertexWithNormalIterator<'a, I> {
    type Item = VertexWithNormal;

    fn next(&mut self) -> Option<Self::Item> {
        let v = VertexWithNormal {
            vertex: match self.vertices.next() {
                Some(v) => v,
                None => {
                    debug_assert_eq!(self.normals.len(), self.normal_count);
                    return None;
                }
            },
            normal: self.normals[self.normal_count],
        };

        if self.vertex_count % 3 == 0 {
            self.normal_count += 1;
        }

        self.vertex_count += 1;

        Some(v)
    }
}

pub struct TriangleIterator<'a, I: Iterator<Item = Point>> {
    vertices: I,
    normals: &'a [Normal],
    normal_cursor: usize,
}

impl<'a, I: Iterator<Item = Point>> TriangleIterator<'a, I> {
    pub fn new(vertices: I, normals: &'a [Normal]) -> Self {
        Self {
            vertices,
            normals,
            normal_cursor: 0,
        }
    }
}

impl<'a, I: Iterator<Item = Point>> Iterator for TriangleIterator<'a, I> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        let v0 = self.vertices.next()?;
        let v1 = self.vertices.next()?;
        let v2 = self.vertices.next()?;

        let normal = self.normals[self.normal_cursor];

        self.normal_cursor += 1;

        Some(Triangle { normal, v0, v1, v2 })
    }
}
