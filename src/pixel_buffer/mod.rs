use material::Material;
use material::Sand;
use std::vec;
use std::mem;

pub type PixelBuffer = Vec<Vec<Cell>>;

#[derive(Clone, Debug)]
pub struct Cell {
    pub contents: Material
}

impl Cell {
    fn default() -> Cell {
        Cell { contents: Material::default() }
    }

    pub fn set_contents(&mut self, new_contents: Material) {
        self.contents = new_contents.clone();
    }
}

pub fn new(width: usize, height: usize) -> PixelBuffer {
    let mut v = Vec::with_capacity(height);
    for _ in 0..height { v.push(vec![Cell::default(); width]) }
    {
        let row = &mut v[10];
        row[10].contents = Material::Sand(Sand::default());
    }
    return v;
}
