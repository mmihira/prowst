use std::collections::HashMap;
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
    // Could make this a vector as well
    // Track what indexes are available wiht a que
    // When something is delted put the index back in the que
    // we only access the que when the brush is called
    mat_register: HashMap<usize, MaterialRecord>,
    uuid_count: usize,
    // Make this a flat Vec and use vector arithmetic to access
    // Will make it faster.
    pub mat_map: Vec<Cell>
}

impl MaterialMap {
    pub fn new(width: usize, height: usize) -> MaterialMap {
        let mut ret = MaterialMap {
            map_width: width,
            map_height: height,
            mat_register: HashMap::new(),
            uuid_count: 0usize,
            mat_map: vec![Cell::default(); width*height]
        };
        ret.mat_register.reserve(width * height);
        ret.add_material(100, 100, Material::Sand);
        ret
    }

    pub fn add_material(&mut self, width: usize, height: usize, material: Material){
        if !self.something_at_index(height, width) {
            let u = self.uuid_count + 1;
            self.uuid_count = u;
            let m = MaterialRecord{ mat: material, state: State::Free };
            self.mat_register.insert(u, m);
            self.mat_map[height*self.map_width + width].contents = Some(u);
        }
    }

    pub fn rgb_of_uuid(&self, uuid: usize) -> RGB {
        Material::rgb(&self.mat_register.get(&uuid).unwrap().mat)
    }

    pub fn change_state_of_uuid(&mut self, uuid: usize, state: State) {
       self.mat_register.get_mut(&uuid).unwrap().state = state;
    }

    pub fn change_state_at_index(&mut self, y: usize, x: usize, state: State) {
        let u = self.mat_map[y * self.map_width + x].contents.unwrap();
        self.change_state_of_uuid(u, state);
    }

    pub fn something_at_index(&self, y: usize, x: usize) -> bool {
        self.mat_map[y * self.map_width + x].contents.is_some()
    }

    pub fn uuid_at_index(&self, y: usize, x: usize) -> usize{
        self.mat_map[y * self.map_width + x].contents.unwrap()
    }

    pub fn contents_at_index (&self, y: usize, x: usize) -> Option<usize>{
        self.mat_map[y * self.map_width + x].contents
    }

    pub fn state_of_material(&self, u: usize) -> State {
        self.mat_register.get(&u).unwrap().state
    }

    pub fn state_at_index (&self, y: usize, x: usize) -> State {
        let u = self.mat_map[y * self.map_width + x].contents.unwrap();
        self.state_of_material(u)
    }

    pub fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        let u = self.mat_map[yfrom*self.map_width + xfrom].contents.unwrap();
        self.mat_map[yto * self.map_width + xto].contents = Some(u);
        self.mat_map[yfrom * self.map_width + xfrom].contents = None;
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
