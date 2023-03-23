use crate::Point;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    /// Returns a bounding box containing no points
    pub fn new() -> Self {
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

    /// Expands the bounding box if it does not currently contain a given point
    pub fn add_point(&mut self, vertex: Point) {
        self.min.x = self.min.x.min(vertex.x);
        self.min.y = self.min.y.min(vertex.y);
        self.min.z = self.min.z.min(vertex.z);

        self.max.x = self.max.x.max(vertex.x);
        self.max.y = self.max.y.max(vertex.y);
        self.max.z = self.max.z.max(vertex.z);
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

    pub fn contains_point(&self, p: Point) -> bool {
        self.max > p && self.min < p
    }
}
