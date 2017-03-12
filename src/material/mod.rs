#[derive(Debug)]
pub struct RGB          { pub red: usize, pub green: usize, pub blue: usize  }
pub struct Sand         { speed: f32, rgb: RGB }
pub struct Water        { speed: f32, rgb: RGB }
pub struct Stone        { speed: f32, rgb: RGB }
pub struct Background   { rgb: RGB }

pub enum Material {
    Sand(Sand),
    Water(Water),
    Stone(Stone),
    Background(Background)
}

impl Material {
    pub fn rgb(&self) -> &RGB {
        match *self {
            Material::Sand(ref x) => &x.rgb,
            Material::Water(ref x) => &x.rgb,
            Material::Stone(ref x) => &x.rgb,
            Material::Background(ref x) => &x.rgb
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Material {
        match *self {
            Material::Sand(_) => Material::Sand(Sand::default()),
            Material::Water(_) => Material::Water(Water::default()),
            Material::Stone(_) => Material::Stone(Stone::default()),
            Material::Background(_) => Material::Background(Background::default())
        }
    }
}

impl Default for Sand {
    fn default() -> Sand {
        Sand {
            speed: 0.10f32,
            rgb: RGB { red: 10, green: 10, blue: 10 }
        }
    }
}

impl Default for Water {
    fn default() -> Water {
        Water {
            speed: 0.10f32,
            rgb: RGB { red: 10, green: 10, blue: 10 }
        }
    }
}

impl Default for Stone {
    fn default() -> Stone {
        Stone {
            speed: 0.10f32,
            rgb: RGB { red: 10, green: 10, blue: 10 }
        }
    }
}

impl Default for Background {
    fn default() -> Background {
        Background {
            rgb: RGB { red: 40, green: 30, blue: 10 }
        }
    }
}


