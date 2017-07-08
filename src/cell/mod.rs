use material::Material;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Cell {
    pub contents: Option<Uuid>
}

impl Cell {
    pub fn default() -> Cell {
        Cell { contents: None }
    }
}
