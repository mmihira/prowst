use std::collections::HashMap;
use std::vec;
use material::Material;
use cell::Cell;
use cell::MaterialRecord;
use material::RGB;
use material::State;

pub struct MaterialMap {
    map_width: usize,
    map_height: usize,
    pub mat_map: Vec<Cell>
}

impl MaterialMap {
    pub fn new(width: usize, height: usize) -> MaterialMap {
        let mut ret = MaterialMap {
            map_width: width,
            map_height: height,
            mat_map: vec![Cell::default(); width*height]
        };
        ret.add_material(100, 100, Material::Sand);
        ret
    }

    pub fn add_material(&mut self, width: usize, height: usize, material: Material){
        if !self.something_at_index(height, width) {
            let m = MaterialRecord{ mat: material, state: State::Free };
            self.mat_map[height*self.map_width + width].contents = Some(m);
        }
    }

    pub fn change_state_at_index(&mut self, y: usize, x: usize, state: State) {
        if let Some(i) = self.mat_map[y * self.map_width + x].contents.as_mut() {
            i.state = state;
        }
    }

    pub fn something_at_index(&self, y: usize, x: usize) -> bool {
        self.mat_map[y * self.map_width + x].contents.is_some()
    }

    pub fn contents_at_index (&self, y: usize, x: usize) -> Option<MaterialRecord>{
        self.mat_map[y * self.map_width + x].contents
    }

    pub fn state_at_index (&self, y: usize, x: usize) -> State {
        self.mat_map[y * self.map_width + x].contents.unwrap().state
    }

    pub fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        {
            let moving = &self.mat_map[yfrom*self.map_width + xfrom].contents.unwrap();
            // To should always be none
            self.mat_map[yto * self.map_width + xto].contents = Some(*moving);
        }
        self.mat_map[yfrom * self.map_width + xfrom].contents = None;
    }

    pub fn copy_rgb_index(&self, offset: usize, buf: &mut [u8], y: usize, x: usize) {
        let rgb = self.mat_map[y * self.map_width + x].contents.unwrap().mat.rgb();
        buf[offset + 0] = rgb.red as u8;
        buf[offset + 1] = rgb.green as u8;
        buf[offset + 2] = rgb.blue as u8;
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
