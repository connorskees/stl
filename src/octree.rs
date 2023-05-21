use std::ops::{Add, Sub};

use crate::{Point, StlFile, Triangle};

type TriangleId = u32;
type OctantId = u32;

/// If an octant has 50 triangles, we split it up into 8 consitutent parts
const TRIANGLE_LIMIT: usize = 50;

/// If an octant has 9 parents, we do not want to split further
const DEPTH_LIMIT: u8 = 8;

/// An octree representing the intersection between two meshes
#[derive(Debug, Clone)]
pub struct Octree {
    pub(crate) octants: Vec<Octant>,
    pub triangles: Vec<Triangle>,
}

#[derive(Debug, Clone)]
pub(crate) struct Octant {
    children: Option<[OctantId; 8]>,
    triangle_ids: Vec<TriangleId>,
    center: Point,
    radius: Point,
    depth: u8,
}

impl Octant {
    fn should_split(&self) -> bool {
        self.triangle_ids.len() >= TRIANGLE_LIMIT
            && self.children.is_none()
            && self.depth < DEPTH_LIMIT
    }

    fn contains_triangle(&self, triangle: Triangle) -> bool {
        self.contains_point(triangle.v0)
            && self.contains_point(triangle.v1)
            && self.contains_point(triangle.v2)
    }

    fn contains_point(&self, point: Point) -> bool {
        self.center.x + self.radius.x >= point.x
            && self.center.x - self.radius.x <= point.x
            && self.center.y + self.radius.y >= point.y
            && self.center.y - self.radius.y <= point.y
            && self.center.z + self.radius.z >= point.z
            && self.center.z - self.radius.z <= point.z
    }
}

/// implementing the below pseudocode in a pseudocode that
/// is closer to rust
/*
let t1s = vec![];
let t2s = vec![];

for t1 in t1s {
    for t2 in t2s {
        for edge in t1 {
            m = t2.intersection(edge)
            if (m.is_some() && )
        }

        for edge in t2 {
            m = t1.intersection(edge)

        }
    }
}

*/

// for each pair of triangles (T1, T2) do
//    for each edge e ∈ T1 do
//        m = Intersection(e, T2);
//        tif (exist_intersection (e, T2) &&
// m∗ = Intersection(e, T2))
// Properties(m) = Properties(m∗) =
// Priority(m, m∗);
// Coordinate(m∗) = Coordinate(m);
// end for
// for each edge e ∈ T2 do
// m = Intersection(e, T1);
// if (exist_intersection (e, T1) &&
// m∗ = Intersection(e, T1))
// Properties(m) = Properties(m∗) =
// Priority(m, m∗);
// Coordinate(m∗) = Coordinate(m);
// end for
// end for

impl Octree {
    pub fn new(mesh: StlFile) -> Self {
        let bbox = mesh.bounding_box();

        let root_octant = Octant {
            children: None,
            triangle_ids: Vec::new(),
            center: bbox.center(),
            radius: bbox.radius(),
            depth: 0,
        };

        let mut octree = Octree {
            octants: vec![root_octant],
            triangles: Vec::new(),
        };

        for triangle in mesh.triangles() {
            octree.insert_triangle(triangle);
        }

        octree
    }

    pub fn new_intersection(mesh_one: StlFile, mesh_two: StlFile) -> Self {
        let bounding_box_one = mesh_one.bounding_box();
        let bounding_box_two = mesh_two.bounding_box();

        let longest_edge = mesh_one.longest_edge().max(mesh_two.longest_edge());

        let mut intersection_bounding_box = bounding_box_one.intersection(&bounding_box_two);

        intersection_bounding_box.extend(longest_edge);

        let root_octant = Octant {
            children: None,
            triangle_ids: Vec::new(),
            center: intersection_bounding_box.center(),
            radius: intersection_bounding_box.radius(),
            depth: 0,
        };

        let mut octree = Octree {
            octants: vec![root_octant],
            triangles: Vec::new(),
        };

        for triangle in mesh_one.triangles() {
            if !intersection_bounding_box.contains_triangle(triangle) {
                continue;
            }

            octree.insert_triangle(triangle);
        }

        for triangle in mesh_two.triangles() {
            if !intersection_bounding_box.contains_triangle(triangle) {
                continue;
            }

            octree.insert_triangle(triangle);
        }

        octree
    }

    fn insert_triangle(&mut self, triangle: Triangle) {
        let triangle_id = self.triangles.len() as u32;

        self.triangles.push(triangle);

        let root = self.root();

        assert!(root.contains_triangle(triangle));

        self.insert_triangle_into_octant(0, triangle_id)
    }

    fn get_octant(&self, octant_id: OctantId) -> &Octant {
        unsafe { self.octants.get_unchecked(octant_id as usize) }
    }

    fn get_triangle(&self, triangle_id: TriangleId) -> Triangle {
        *unsafe { self.triangles.get_unchecked(triangle_id as usize) }
    }

    fn root(&self) -> &Octant {
        self.get_octant(0)
    }

    fn get_octant_mut(&mut self, octant_id: OctantId) -> &mut Octant {
        unsafe { self.octants.get_unchecked_mut(octant_id as usize) }
    }

    fn insert_octant(&mut self, octant: Octant) -> OctantId {
        let id = self.octants.len() as u32;

        self.octants.push(octant);

        id
    }

    /// Invariant: we must know that the triangle is contained inside the octant
    fn insert_triangle_into_octant(&mut self, octant_id: OctantId, triangle_id: TriangleId) {
        let triangle = self.get_triangle(triangle_id);
        let octant = self.get_octant(octant_id);

        assert!(octant.contains_triangle(triangle));

        if octant.should_split() {
            self.split_octant(octant_id);
        }

        let octant = self.get_octant_mut(octant_id);

        if let Some(children) = octant.children {
            for child in children {
                let child_octant = self.get_octant(child);

                if child_octant.contains_triangle(triangle) {
                    self.insert_triangle_into_octant(child, triangle_id);

                    return;
                }
            }
        }

        let octant = self.get_octant_mut(octant_id);
        octant.triangle_ids.push(triangle_id);
    }

    fn split_octant(&mut self, octant_id: OctantId) {
        let octant = self.get_octant(octant_id);

        assert!(octant.children.is_none());
        assert!(octant.should_split());

        let radius = octant.radius / 2.0;
        let depth = octant.depth + 1;

        let center_fns: [(
            fn(f32, f32) -> f32,
            fn(f32, f32) -> f32,
            fn(f32, f32) -> f32,
        ); 8] = [
            (Add::add, Sub::sub, Add::add),
            (Add::add, Add::add, Add::add),
            (Add::add, Add::add, Sub::sub),
            (Add::add, Sub::sub, Sub::sub),
            (Sub::sub, Sub::sub, Sub::sub),
            (Sub::sub, Sub::sub, Add::add),
            (Sub::sub, Add::add, Add::add),
            (Sub::sub, Add::add, Sub::sub),
        ];

        let centers = center_fns.map(|(x, y, z)| Point {
            x: x(octant.center.x, radius.x),
            y: y(octant.center.y, radius.y),
            z: z(octant.center.z, radius.z),
        });

        let mut octants = centers.map(|center| Octant {
            children: None,
            radius,
            center,
            depth,
            triangle_ids: Vec::new(),
        });

        let triangles = octant.triangle_ids.iter().copied();

        let mut outer_triangle_ids = Vec::new();

        'outer: for triangle_id in triangles {
            let triangle = self.get_triangle(triangle_id);

            for octant in &mut octants {
                if octant.contains_triangle(triangle) {
                    octant.triangle_ids.push(triangle_id);

                    continue 'outer;
                }
            }

            outer_triangle_ids.push(triangle_id);
        }

        let children = octants.map(|octant| self.insert_octant(octant));

        let octant = self.get_octant_mut(octant_id);
        octant.children = Some(children);
        octant.triangle_ids = outer_triangle_ids;
    }
}

#[cfg(test)]
mod test {
    use crate::{octree::Octree, Normal, Point, StlFile, Triangle};

    #[test]
    fn insert_one_triangle_goes_to_root() {
        let mut file = StlFile::new();

        let origin = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        file.add_triangle(Triangle {
            normal: Normal {
                i: 0.0,
                j: 0.0,
                k: 0.0,
            },
            v0: origin,
            v1: origin,
            v2: origin,
        });

        let octree = Octree::new(file);

        assert_eq!(octree.octants.len(), 1);
        assert_eq!(octree.triangles.len(), 1);
    }

    #[test]
    fn large_stl_file() {
        let mut file = StlFile::from_path("Wikipedia_puzzle_globe_3D_render.stl").unwrap();

        let octree = Octree::new(file);
    }
}
