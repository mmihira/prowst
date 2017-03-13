use material;
use std::vec;
use std::mem;

pub type PixelBuffer = Vec<Vec<material::Material>>;

pub fn new(width: usize, height: usize) -> PixelBuffer {
    let mut v = Vec::with_capacity(height);
    for _ in 0..height {
        v.push(vec![material::Material::Background(material::Background::default()); width]);
    }
    {
        let row = &mut v[10];
        mem::replace(&mut row[10], material::Material::Sand(material::Sand::default()));
    }
    {
        let row = &mut v[10];
        mem::replace(&mut row[30], material::Material::Sand(material::Sand::default()));
    }
    return v;
}
