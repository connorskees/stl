use crate::{Point, Triangle};

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    pub(crate) fn init() -> Self {
        Self {
            min: Point {
                x: f32::INFINITY,
                y: f32::INFINITY,
                z: f32::INFINITY,
            },
            max: Point {
                x: f32::NEG_INFINITY,
                y: f32::NEG_INFINITY,
                z: f32::NEG_INFINITY,
            },
        }
    }

    pub(crate) fn add_point(&mut self, vertex: Point) {
        self.min.x = self.min.x.min(vertex.x as f32);
        self.min.y = self.min.y.min(vertex.y as f32);
        self.min.z = self.min.z.min(vertex.z as f32);

        self.max.x = self.max.x.max(vertex.x as f32);
        self.max.y = self.max.y.max(vertex.y as f32);
        self.max.z = self.max.z.max(vertex.z as f32);
    }

    pub fn center(&self) -> Point {
        Point {
            x: (self.min.x + self.max.x) / 2.0,
            y: (self.min.y + self.max.y) / 2.0,
            z: (self.min.z + self.max.z) / 2.0,
        }
    }

    pub fn delta(&self) -> Point {
        Point {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
            z: self.max.z - self.min.z,
        }
    }

    pub fn radius(&self) -> Point {
        self.delta().abs()
    }

    pub fn contains_point(&self, p: Point) -> bool {
        self.max.x >= p.x
            && p.x >= self.min.x
            && self.max.y >= p.y
            && p.y >= self.min.y
            && self.max.z >= p.z
            && p.z >= self.min.z
    }

    /// Returns whether the bounding box contains the _entire_ triangle
    pub fn contains_triangle(&self, triangle: Triangle) -> bool {
        self.contains_point(triangle.v0)
            && self.contains_point(triangle.v1)
            && self.contains_point(triangle.v2)
    }

    /// Returns a bounding box that represents the volume encompassed by either
    /// bounding box
    pub fn union(&self, other: &Self) -> Self {
        BoundingBox {
            max: Point {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
                z: self.max.z.max(other.max.z),
            },
            min: Point {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
                z: self.min.z.min(other.min.z),
            },
        }
    }

    /// Returns a bounding box that represents the volume encompassed by both
    /// bounding boxes
    pub fn intersection(&self, other: &Self) -> Self {
        BoundingBox {
            max: Point {
                x: self.max.x.min(other.max.x),
                y: self.max.y.min(other.max.y),
                z: self.max.z.min(other.max.z),
            },
            min: Point {
                x: self.min.x.max(other.min.x),
                y: self.min.y.max(other.min.y),
                z: self.min.z.max(other.min.z),
            },
        }
    }

    /// Increase the radius of the bounding box by `len`
    pub fn extend(&mut self, len: f32) {
        self.max.x += len;
        self.max.y += len;
        self.max.z += len;

        self.min.x -= len;
        self.min.y -= len;
        self.min.z -= len;
    }
}

#[cfg(test)]
mod test {
    use crate::{BoundingBox, Point};

    #[test]
    fn bounding_box_contains_point() {
        let mut bbox = BoundingBox::init();

        bbox.add_point(Point::new(0.0, 0.0, 0.0));
        bbox.add_point(Point::new(1.0, 1.0, 1.0));

        assert!(bbox.contains_point(Point::new(0.5, 0.5, 0.5)));
        assert!(bbox.contains_point(Point::new(0.75, 0.75, 0.75)));
        assert!(bbox.contains_point(Point::new(0.001, 0.001, 0.001)));

        assert!(!bbox.contains_point(Point::new(1.5, 0.5, 0.5)));
        assert!(!bbox.contains_point(Point::new(0.75, 1.75, 0.75)));
        assert!(!bbox.contains_point(Point::new(0.001, 5.001, 0.001)));
    }
}
