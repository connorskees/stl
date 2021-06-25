use std::collections::{hash_map::Entry, HashMap};

use crate::{Point, StlFile, Triangle};

pub(crate) struct UnionFind {
    ids: Vec<usize>,
    elements: HashMap<Point, usize>,
    number_of_elements: usize,
    triangles: Vec<Triangle>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            ids: (0..size).collect(),
            elements: HashMap::new(),
            number_of_elements: 0,
            triangles: Vec::new(),
        }
    }

    pub fn files(&mut self) -> Vec<StlFile> {
        let mut map: HashMap<_, StlFile> = HashMap::new();

        for triangle in self.triangles.clone() {
            let root = self.root(triangle.v0);

            match map.entry(root) {
                Entry::Occupied(mut val) => {
                    val.get_mut().add_triangle(triangle);
                }
                Entry::Vacant(ptr) => {
                    let mut file = StlFile::new();
                    file.add_triangle(triangle);
                    ptr.insert(file);
                }
            }
        }

        map.into_iter().map(|(_, v)| v).collect()
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.connect_points(triangle.v0, triangle.v1);
        self.connect_points(triangle.v0, triangle.v2);

        self.triangles.push(triangle);
    }

    fn get_id_for_point(&mut self, point: Point) -> usize {
        let idx = self.elements.len();
        match self.elements.entry(point) {
            Entry::Occupied(val) => *val.get(),
            Entry::Vacant(ptr) => {
                ptr.insert(idx);
                self.number_of_elements += 1;
                idx
            }
        }
    }

    fn root(&mut self, point: Point) -> usize {
        let mut id = self.get_id_for_point(point);

        let mut root = id;

        while root != self.ids[root] {
            root = self.ids[root];
        }

        while root != id {
            let next = self.ids[id];
            self.ids[next] = root;
            id = next;
        }

        root
    }

    fn connect_points(&mut self, p1: Point, p2: Point) {
        let root_p1 = self.root(p1);
        let root_p2 = self.root(p2);

        if root_p1 == root_p2 {
            return;
        }

        self.number_of_elements -= 1;

        self.ids[root_p1] = root_p2;
    }
}
