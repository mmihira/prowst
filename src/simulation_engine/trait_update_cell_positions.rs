use SimulationEngine;
use material::State;
use material::Material;
use time;
use rand;
use rand::distributions::{IndependentSample, Range};
use window;

pub trait UpdateCellPositions {
    fn update_cell_positions(&mut self, elapsed: &time::Duration);
    fn try_move_side_down(&mut self, y: usize, x: usize);
    fn handle_sand(&mut self, y: usize, x: usize);
    fn handle_water(&mut self, y: usize, x: usize);
    fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, yto: usize);
}

impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self, elapsed: &time::Duration) {
        self.map.reset_states();

        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                if self.map.something_at_index(y, x) &&
                   self.map.state_at_index(y, x) == State::Free &&
                   y < (self.buffer_height - 1) {
                       match self.map.material_at_index(y, x) {
                           Material::Sand => self.handle_sand(y, x),
                           Material::Water => (),
                           Material::Stone => ()
                        }
                   }
            }
        }
    }

    fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        self.map.move_material(yfrom, xfrom, yto, xto);
        let offset_from = yfrom * window::SCREEN_WIDTH*3 + xfrom * 3;
        let offset_to = yto * window::SCREEN_WIDTH*3 + xto * 3;

        let tmp_red = self.pixel_buffer[offset_to + 0];
        let tmp_green = self.pixel_buffer[offset_to + 1];
        let tmp_blue = self.pixel_buffer[offset_to + 2];

        self.pixel_buffer[offset_to + 0] = self.pixel_buffer[offset_from + 0];
        self.pixel_buffer[offset_to + 1] = self.pixel_buffer[offset_from + 1];
        self.pixel_buffer[offset_to + 2] = self.pixel_buffer[offset_from + 2];

        self.pixel_buffer[offset_from + 0] = tmp_red;
        self.pixel_buffer[offset_from + 1] = tmp_green;
        self.pixel_buffer[offset_from + 2] = tmp_blue;
    }

    fn handle_sand(&mut self, y: usize, x: usize) {
        if y < (self.buffer_height - 2) &&
           self.map.something_at_index(y + 1, x) &&
           self.map.something_at_index(y + 2, x) &&
           x > 0 && x < self.buffer_width-1 {
            self.try_move_side_down(y, x);
        } else if !self.map.something_at_index(y + 1, x) {
            self.move_material(y, x, y + 1, x);
            self.map.change_state_at_index(y + 1, x, State::Set);
        } else {
            self.map.change_state_at_index(y, x, State::Set);
        }
    }

    fn handle_water(&mut self, y:usize, x:usize) {
    }

    fn try_move_side_down(&mut self,y: usize,x: usize){
        if rand::random::<bool>() {
            if !self.map.something_at_index(y + 1, x + 1) {
                self.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x - 1) {
                self.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        } else {
            if !self.map.something_at_index(y + 1, x - 1) {
                self.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x + 1) {
                self.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        }
    }
}

