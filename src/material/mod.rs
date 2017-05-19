#[derive(PartialEq, Clone, Debug)]
pub struct RGB  { pub red: u8, pub green: u8, pub blue: u8 }

#[derive(PartialEq, Clone, Debug)]
pub enum Material {
    Sand,
    Water,
    Stone,
    Background
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Free,
    Set,
    Dead
}

// Make the enum an actual ename without having a class as an argument
// Then actually have a method which return properties of the enum
// using a match statement
// Define the properties of the material inside the function
// Only the material attribute information requires will be returned at any one time
/*
 * For example fn rgb_for_material(material: Material) -> RGB { match material { ... } }
 * like that
 */

impl Material {
    pub fn rgb(&self) -> RGB {
        match *self {
            Material::Sand => RGB { red: 255, green: 255, blue: 255 },
            Material::Water => RGB { red: 255, green: 255, blue: 255 },
            Material::Stone => RGB { red: 255, green: 255, blue: 255 },
            Material::Background => RGB { red: 0, green: 0, blue: 0 },
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Material {
        match *self {
            Material::Sand(ref x) => Material::Sand(x.clone()),
            Material::Water(_) => Material::Water(Water::default()),
            Material::Stone(_) => Material::Stone(Stone::default()),
            Material::Background(_) => Material::Background(Background::default())
        }
    }
}
