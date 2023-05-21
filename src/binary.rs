use crate::{Normal, StlFile};

#[derive(Debug)]
pub struct BinaryParser<'a> {
    buffer: &'a [u8],
    cursor: usize,
    normals: Vec<Normal>,
    vertices: Vec<f32>,
    num_of_facets: u32,
    header: &'a [u8],
}

impl<'a> BinaryParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Result<Self, ()> {
        if buffer.len() < 80 {
            return Err(());
        }

        let mut bin_parser = Self {
            buffer,
            cursor: 0,
            num_of_facets: 0,
            normals: Vec::new(),
            vertices: Vec::new(),
            header: &[],
        };

        let header = bin_parser.read_header();
        let num_of_facets = bin_parser.read_u32_le();

        bin_parser.header = header;
        bin_parser.num_of_facets = num_of_facets;
        bin_parser.normals.reserve(num_of_facets as usize * 3);
        bin_parser.vertices.reserve(num_of_facets as usize * 3 * 3);

        Ok(bin_parser)
    }

    pub fn parse(mut self) -> StlFile {
        for _ in 0..self.num_of_facets {
            self.read_normal();

            self.read_vertex();
            self.read_vertex();
            self.read_vertex();

            // skip 2-byte short after triangle
            self.cursor += 2;
        }

        StlFile {
            normals: self.normals,
            vertices: self.vertices,
        }
    }

    fn read_header(&mut self) -> &'a [u8] {
        self.cursor += 80;
        &self.buffer[..80]
    }

    fn read_u32_le(&mut self) -> u32 {
        self.cursor += 4;

        assert!(self.cursor <= self.buffer.len());

        unsafe {
            u32::from_le_bytes([
                *self.buffer.get_unchecked(self.cursor - 4),
                *self.buffer.get_unchecked(self.cursor - 3),
                *self.buffer.get_unchecked(self.cursor - 2),
                *self.buffer.get_unchecked(self.cursor - 1),
            ])
        }
    }

    fn read_f32_le(&mut self) -> f32 {
        self.cursor += 4;

        assert!(self.cursor <= self.buffer.len());

        // SAFETY: we assert above that these indices are valid
        //
        // for some reason, the bounds checks are not optimized away
        unsafe {
            f32::from_le_bytes([
                *self.buffer.get_unchecked(self.cursor - 4),
                *self.buffer.get_unchecked(self.cursor - 3),
                *self.buffer.get_unchecked(self.cursor - 2),
                *self.buffer.get_unchecked(self.cursor - 1),
            ])
        }
    }

    fn read_normal(&mut self) {
        let i = self.read_f32_le();
        let j = self.read_f32_le();
        let k = self.read_f32_le();

        self.normals.push(Normal { i, j, k });
    }

    fn read_vertex(&mut self) {
        let x = self.read_f32_le();
        let y = self.read_f32_le();
        let z = self.read_f32_le();

        self.vertices.push(x);
        self.vertices.push(y);
        self.vertices.push(z);
    }
}
