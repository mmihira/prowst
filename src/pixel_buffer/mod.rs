use material::Material;
use std::vec;
use std::mem;
use cell::Cell;

pub type PixelBuffer = Vec<Vec<Cell>>;

// pub fn new(width: usize, height: usize) -> PixelBuffer {
//     let mut v = Vec::with_capacity(height);
//     for _ in 0..height { v.push(vec![Cell::default(); width]) }
//     {
//         let row = &mut v[10];
//         // row[10].contents = Material::Sand(Sand::default());
//     }
//     return v;
// }
