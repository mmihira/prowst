use std::collections::HashMap;
use uuid::Uuid;
use std::vec;
use material::Material;
use cell::Cell;
use material::RGB;

pub struct MaterialMap {
    map_width: usize,
    map_height: usize,
    mat_register: HashMap<Uuid, Box<Material>>,
    pub mat_map: Vec<Vec<Cell>>
}

impl MaterialMap {
    pub fn new(width: usize, height: usize) -> MaterialMap {
        let ret = MaterialMap {
            map_width: width,
            map_height: height,
            mat_register: HashMap::new(),
            mat_map: {
                let mut v = Vec::with_capacity(height);
                for _ in 0..height { v.push(vec![Cell::default(); width]) }
                v
            }
        };
        ret
    }

    pub fn add_material(&mut self, width: usize, height: usize, material: Material){
        let u = Uuid::new_v4();
        let m = Box::new(material);
        self.mat_register.insert(u, m);
        self.mat_map[height][width].contents = Some(u);
    }

    pub fn rgb_of_uuid(&self, uuid: Uuid) -> RGB {
        Material::rgb(self.mat_register.get(&uuid).unwrap())
    }
}


