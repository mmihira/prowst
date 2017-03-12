use material;
use std::vec;

pub type PixelBuffer = Vec<Vec<material::Material>>;

pub fn new(width: usize, height: usize) -> PixelBuffer {
    let mut v = Vec::with_capacity(height);
    for _ in 0..height {
        v.push(vec![material::Material::Background(material::Background::default()); width]);
    }
    return v;
}
