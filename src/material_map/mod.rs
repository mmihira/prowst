use std::collections::HashMap;
use uuid::Uuid;
use std::vec;
use material::Material;
use cell::Cell;
use material::RGB;
use material::State;

pub struct MaterialRecord {
    mat: Material,
    state: State
}

pub struct MaterialMap {
    map_width: usize,
    map_height: usize,
    mat_register: HashMap<Uuid, MaterialRecord>,
    pub mat_map: Vec<Vec<Cell>>
}

impl MaterialMap {
    pub fn new(width: usize, height: usize) -> MaterialMap {
        let mut ret = MaterialMap {
            map_width: width,
            map_height: height,
            mat_register: HashMap::new(),
            mat_map: {
                let mut v = Vec::with_capacity(height);
                for _ in 0..height { v.push(vec![Cell::default(); width]) }
                v
            }
        };
        ret.add_material(100,100, Material::Sand);
        ret
    }

    pub fn add_material(&mut self, width: usize, height: usize, material: Material){
        if !self.something_at_index(height, width) {
            let u = Uuid::new_v4();
            let m = MaterialRecord{ mat: material, state: State::Free };
            self.mat_register.insert(u, m);
            self.mat_map[height][width].contents = Some(u);
        }
    }

    pub fn rgb_of_uuid(&self, uuid: Uuid) -> RGB {
        Material::rgb(&self.mat_register.get(&uuid).unwrap().mat)
    }

    pub fn change_state_of_uuid(&mut self, uuid: Uuid, state: State) {
       self.mat_register.get_mut(&uuid).unwrap().state = state;
    }

    pub fn change_state_at_index(&mut self, y: usize, x: usize, state: State) {
        let u = self.mat_map[y][x].contents.unwrap();
        self.change_state_of_uuid(u, state);
    }

    pub fn something_at_index(&self, y: usize, x: usize) -> bool {
        self.mat_map[y][x].contents.is_some()
    }

    pub fn uuid_at_index(&self, y: usize, x: usize) -> Uuid {
        self.mat_map[y][x].contents.unwrap()
    }

    pub fn state_of_material(&self, u: Uuid) -> State {
        self.mat_register.get(&u).unwrap().state
    }

    pub fn state_at_index (&self, y: usize, x: usize) -> State {
        let u = self.mat_map[y][x].contents.unwrap();
        self.state_of_material(u)
    }

    pub fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        let u = self.mat_map[yfrom][xfrom].contents.unwrap();
        self.mat_map[yto][xto].contents = Some(u);
        self.mat_map[yfrom][xfrom].contents = None;
    }

    pub fn reset_states(&mut self) {
        for y in 0..self.map_height{
            for x in 0..self.map_width{
                if self.something_at_index(y, x) {
                    self.change_state_at_index(y, x, State::Free);
                }
            }
        }
    }
}
