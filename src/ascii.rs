use crate::{Normal, StlFile};

#[derive(Debug)]
pub struct AsciiParser<'a> {
    buffer: &'a [u8],
    cursor: usize,
    normals: Vec<Normal>,
    vertices: Vec<f32>,
}

impl<'a> AsciiParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Result<Self, ()> {
        let mut ascii_parser = Self {
            buffer,
            cursor: 0,
            normals: Vec::new(),
            vertices: Vec::new(),
        };

        ascii_parser.skip_whitespace();
        ascii_parser.expect_bytes(b"solid ")?;
        ascii_parser.skip_whitespace();

        Ok(ascii_parser)
    }

    pub fn parse(mut self) -> StlFile<'a> {
        let name = self.read_string();

        self.skip_whitespace();

        while !self.consume_if_next_is_bytes(b"endsolid") {
            self.read_triangle().unwrap();
        }

        StlFile {
            header: name,
            normals: self.normals,
            vertices: self.vertices,
        }
    }

    fn expect_bytes(&mut self, s: &'static [u8]) -> Result<(), ()> {
        if &self.buffer[self.cursor..(self.cursor + s.len())] != s {
            return Err(());
        }

        self.cursor += s.len();

        Ok(())
    }

    fn consume_if_next_is_bytes(&mut self, s: &'static [u8]) -> bool {
        if &self.buffer[self.cursor..(self.cursor + s.len())] == s {
            self.cursor += s.len();
            true
        } else {
            false
        }
    }

    fn next_is_whitespace(&self) -> bool {
        self.buffer[self.cursor].is_ascii_whitespace()
    }

    fn consume_next_if_whitespace(&mut self) -> bool {
        let next_is_whitespace = self.next_is_whitespace();

        if next_is_whitespace {
            self.cursor += 1;
        }

        next_is_whitespace
    }

    fn skip_whitespace(&mut self) {
        while self.consume_next_if_whitespace() {}
    }

    fn read_string(&mut self) -> &'a [u8] {
        let cursor_start = self.cursor;

        while !self.next_is_whitespace() {
            self.cursor += 1;
        }

        &self.buffer[cursor_start..self.cursor]
    }

    fn read_triangle(&mut self) -> Result<(), ()> {
        self.skip_whitespace();
        self.expect_bytes(b"facet")?;
        self.skip_whitespace();
        self.expect_bytes(b"normal")?;
        self.skip_whitespace();

        self.read_normal();

        self.skip_whitespace();
        self.expect_bytes(b"outer")?;
        self.skip_whitespace();
        self.expect_bytes(b"loop")?;
        self.skip_whitespace();

        for _ in 0..3 {
            self.read_vertex()?;
        }

        self.expect_bytes(b"endloop")?;
        self.skip_whitespace();

        self.expect_bytes(b"endfacet")?;
        self.skip_whitespace();

        Ok(())
    }

    fn read_normal(&mut self) {
        let i = self.read_float();
        let j = self.read_float();
        let k = self.read_float();

        self.normals.push(Normal { i, j, k });
    }

    fn read_float(&mut self) -> f32 {
        let cursor_start = self.cursor;

        while !self.next_is_whitespace() {
            self.cursor += 1;
        }

        let float = fast_float::parse(&self.buffer[cursor_start..self.cursor]).unwrap();

        self.skip_whitespace();

        float
    }

    fn read_vertex(&mut self) -> Result<(), ()> {
        self.expect_bytes(b"vertex")?;
        for _ in 0..3 {
            self.skip_whitespace();
            let cursor_start = self.cursor;

            while !self.next_is_whitespace() {
                self.cursor += 1;
            }

            self.vertices
                .push(fast_float::parse(&self.buffer[cursor_start..self.cursor]).unwrap());

            self.skip_whitespace();
        }

        Ok(())
    }
}
