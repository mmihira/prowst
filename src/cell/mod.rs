use material::Material;

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub contents: Option<usize>
}

impl Cell {
    pub fn default() -> Cell {
        Cell { contents: None }
    }
}
