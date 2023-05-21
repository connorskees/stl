struct BoundingVolumeHierarchy {
    root: Node,
}

enum Node {
    Node {
        bbox: BoundingBox,
        left: Box<Self>,
        right: Box<Self>,
    },
    Leaf(Vec<Point>),
}

struct Ray {
    origin: Point,
    direction: Point,
}

struct HitInfo {}

impl BoundingVolumeHierarchy {
    pub fn find_nearest_hit(&self) -> ! {
        todo!()
    }
}
