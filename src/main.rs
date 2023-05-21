use std::{
    array,
    collections::{hash_map::Entry, HashMap, HashSet},
    fs,
    iter::FromIterator,
};

use stl::{DoublePoint, Normal, Octree, Point, StlFile, Triangle, VertexWithNormal};

fn main() {
    //     // let path = "./BiteScan.stl";
    //     // let path = "./teapot.stl";
    //     // let path = "./tt";
    //     // let path = "./0";
    //     // let path = "./15";
    //     // let path = "./separate.stl";
    //     // let path = "/home/connor/Downloads/3dslash-23 Jun 21.stl";

    //     // let stl_file = StlFile::from_path(path).unwrap();

    //     // let octree = Octree::new(&stl_file);

    let x = StlFile::from_path("/home/connor/Downloads/plan-stls/UpperJawScan.stl").unwrap();

    let ib = x.index_buffer_vertex_and_normal();

    // let mut tooth = StlFile::from_path("./0").unwrap();
    // let jaw = StlFile::from_path("./25").unwrap();

    // let octree = Octree::new_intersection(tooth, jaw);

    // dbg!(octree.triangles.len());

    //     // tooth.join(&jaw);

    //     // let combined = tooth;

    //     let combined = StlFile {
    //         vertices: std::iter::repeat(0.0)
    //             .zip(std::iter::repeat(5.0))
    //             .flat_map(|(a, b)| [a, b])
    //             .take(100)
    //             .collect(),
    //         normals: vec![Normal::zero(); 100],
    //     };

    //     let octree = Octree::new(&combined);

    //     dbg!(octree);

    //     // let mut file = fs::File::create("./combined").unwrap();
    //     // combined.write_binary(&mut file);

    //     // for (idx, stl_file) in stl_file.split_islands().into_iter().enumerate() {
    //     //     let octree = Octree::new(&stl_file);

    //     //     let mut file = fs::File::create(format!("./{}", idx)).unwrap();
    //     //     stl_file.write_binary(&mut file).unwrap();
    //     // }
}

// fn convert_point(p: Point) -> DoublePoint {
//     DoublePoint {
//         x: p.x as f64,
//         y: p.y as f64,
//         z: p.z as f64,
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// #[repr(transparent)]
// // todo: use u32
// struct OctantId(usize);

// const MAX_DEPTH: usize = 9;
// const TRIANGLE_LIMIT: usize = 5;

// #[derive(Debug)]
// struct Octree {
//     octants: Vec<Octant>,
//     triangles: Vec<Triangle>,
//     root: OctantId,
// }

// impl Octree {
//     pub fn new(stl: &StlFile) -> Self {
//         let bbox = stl.bounding_box();

//         let delta = bbox.delta();

//         let radius = delta.x.max(delta.y).max(delta.z);

//         let mut root = Octant::new(bbox.center(), radius);

//         let triangles: Vec<Triangle> = stl.triangles().collect();

//         root.triangle_ids = (0..triangles.len()).collect();

//         let mut tree = Octree {
//             octants: vec![root],
//             triangles,
//             root: OctantId(0),
//         };

//         tree.build();

//         tree
//     }

//     fn root(&self) -> &Octant {
//         &self.octants[self.root.0]
//     }

//     fn build(&mut self) {
//         if self.root().triangle_count() < TRIANGLE_LIMIT {
//             return;
//         }

//         let mut to_split = vec![self.root];

//         while let Some(octant_id) = to_split.pop() {
//             let octant = &self.octants[octant_id.0];

//             if octant.depth >= MAX_DEPTH {
//                 continue;
//             }

//             for child_id in array::IntoIter::new(self.split_octant(octant_id)) {
//                 if child_id.0 == 360 {
//                     // dbg!(octant.depth);
//                 }
//                 if self.octants[child_id.0].triangle_count() > TRIANGLE_LIMIT {
//                     to_split.push(child_id);
//                 }
//             }
//         }
//     }

//     fn split_octant(&mut self, octant_id: OctantId) -> [OctantId; 8] {
//         let Octant {
//             radius,
//             center,
//             depth,
//             ..
//         } = self.octants[octant_id.0];
//         let new_radius = radius / 2.0;

//         let mut children = [OctantId(0); 8];

//         for i in 0..8 {
//             let factor = get_octant_factor_from_idx(i);

//             let new_center = DoublePoint {
//                 x: center.x + new_radius * factor.x,
//                 y: center.y + new_radius * factor.y,
//                 z: center.z + new_radius * factor.z,
//             };

//             let mut octant = Octant::new(new_center, new_radius);

//             octant.parent = Some(octant_id);
//             octant.depth = depth + 1;

//             let octant_id = OctantId(self.octants.len());

//             self.octants.push(octant);

//             children[i] = octant_id;
//         }

//         self.octants[octant_id.0].children = Some(children);

//         for triangle_id in std::mem::take(&mut self.octants[octant_id.0].triangle_ids) {
//             let triangle = self.triangles[triangle_id];

//             let v0_idx = get_octant_cell_index(
//                 triangle.v0.x as f64 - center.x,
//                 triangle.v0.y as f64 - center.y,
//                 triangle.v0.z as f64 - center.z,
//             );
//             let v1_idx = get_octant_cell_index(
//                 triangle.v1.x as f64 - center.x,
//                 triangle.v1.y as f64 - center.y,
//                 triangle.v1.z as f64 - center.z,
//             );
//             let v2_idx = get_octant_cell_index(
//                 triangle.v2.x as f64 - center.x,
//                 triangle.v2.y as f64 - center.y,
//                 triangle.v2.z as f64 - center.z,
//             );

//             if v0_idx == v1_idx && v1_idx == v2_idx {
//                 debug_assert!(self.octants[children[v0_idx].0].contains_triangle(triangle));
//                 self.octants[children[v0_idx].0]
//                     .triangle_ids
//                     .push(triangle_id);
//             } else {
//                 // let v0 = self.octants[children[v0_idx].0].contains_triangle(triangle);
//                 // let v1 = self.octants[children[v1_idx].0].contains_triangle(triangle);
//                 // let v2 = self.octants[children[v2_idx].0].contains_triangle(triangle);
//                 // assert!(!(v0 || v1 || v2));
//                 self.octants[octant_id.0].triangle_ids.push(triangle_id);
//             }
//         }

//         children
//     }
// }

// type Vec3 = DoublePoint;

// #[derive(Debug)]
// struct Octant {
//     parent: Option<OctantId>,
//     children: Option<[OctantId; 8]>,
//     center: DoublePoint,
//     triangle_ids: Vec<usize>,
//     depth: usize,
//     radius: f64,
// }

// impl Octant {
//     pub fn new(center: DoublePoint, radius: f64) -> Self {
//         Self {
//             center,
//             radius,
//             depth: 0,
//             parent: None,
//             children: None,
//             triangle_ids: Vec::new(),
//         }
//     }

//     pub fn triangle_count(&self) -> usize {
//         self.triangle_ids.len()
//     }

//     pub fn len(&self) -> usize {
//         self.triangle_ids.len()
//     }

//     fn contains_triangle(&self, triangle: Triangle) -> bool {
//         self.contains_point(convert_point(triangle.v0))
//             && self.contains_point(convert_point(triangle.v1))
//             && self.contains_point(convert_point(triangle.v2))
//     }

//     fn contains_point(&self, point: DoublePoint) -> bool {
//         (point.x >= (self.center.x - self.radius) && point.x <= (self.center.x + self.radius))
//             && (point.y >= (self.center.y - self.radius)
//                 && point.y <= (self.center.y + self.radius))
//             && (point.z >= (self.center.z - self.radius)
//                 && point.z <= (self.center.z + self.radius))
//     }
// }

// fn get_octant_factor_from_idx(idx: usize) -> Vec3 {
//     Vec3 {
//         x: match idx & 0b001 {
//             0b000 => 1.0,
//             0b001 => -1.0,
//             _ => unreachable!(),
//         },
//         y: match idx & 0b010 {
//             0b000 => 1.0,
//             0b010 => -1.0,
//             _ => unreachable!(),
//         },
//         z: match idx & 0b100 {
//             0b000 => 1.0,
//             0b100 => -1.0,
//             _ => unreachable!(),
//         },
//     }
// }

// #[inline]
// // todo: investigate morton encoding
// fn get_octant_cell_index(x: f64, y: f64, z: f64) -> usize {
//     match (
//         z.is_sign_positive(),
//         y.is_sign_positive(),
//         x.is_sign_positive(),
//     ) {
//         (true, true, true) => 0,
//         (true, true, false) => 1,
//         (true, false, true) => 2,
//         (true, false, false) => 3,
//         (false, true, true) => 4,
//         (false, true, false) => 5,
//         (false, false, true) => 6,
//         (false, false, false) => 7,
//     }
// }
