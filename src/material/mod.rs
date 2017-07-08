#[derive(Clone, Debug)]
pub struct RGB  { pub red: usize, pub green: usize, pub blue: usize  }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State  {
    Free,
    Set,
    Dead
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialAttribute { state: State }

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Sand(MaterialAttribute),
    Water(MaterialAttribute),
    Stone(MaterialAttribute)
}

impl Material{
    pub fn rgb(&self) -> RGB {
        match *self {
            Material::Sand(_) => RGB{ red: 255, green: 255, blue: 255 },
            Material::Water(_) => RGB{ red: 255, green: 255, blue: 255 },
            Material::Stone(_) => RGB{ red: 255, green: 255, blue: 255 }
        }
    }

    pub fn  speed(&self) -> f32 {
        match *self {
            Material::Sand(_) => 0.1f32,
            Material::Water(_) => 0.1f32,
            Material::Stone(_) => 0.1f32
        }
    }

    pub fn def_sand() -> Material {
        Material::Sand(MaterialAttribute{state: State::Free})
    }
}


// impl Clone for Material {
//     fn clone(&self) -> Material {
//         match *self {
//             Material::Sand(ref x) => Material::Sand(x.clone()),
//             Material::Water(_) => Material::Water(Water::default()),
//             Material::Stone(_) => Material::Stone(Stone::default()),
//             Material::Background(_) => Material::Background(Background::default())
//         }
//     }
// }


