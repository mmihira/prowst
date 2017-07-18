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
    fn move_water_sideways(&mut self, y: usize, x: usize);
    fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, yto: usize);
    fn remove_material(&mut self, y: usize, x: usize);
    fn bulk_move(&mut self, y: usize, x: usize);
    fn block_to_right(&self, y: usize, x: usize, count: usize) -> bool;
    fn block_to_left(&self, y: usize, x: usize, count: usize) -> bool;
    fn block_above(&self, y: usize, x: usize, count: usize) -> bool;
    fn update_loop_inner(&mut self, y: usize, x: usize);
}

impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self, elapsed: &time::Duration) {
        self.map.reset_states();
        // println!("p: {}, t: {}, f: {}", self.p_count, self.t_count, self.p_count - self.t_count );
        for y in 0..self.buffer_height {
            if rand::random::<bool>() {
                for x in (0..self.buffer_width) {
                    self.update_loop_inner(y, x);
                }
            } else {
                for x in (0..self.buffer_width).rev() {
                    self.update_loop_inner(y, x);
                }
            }
        }
    }

    fn update_loop_inner(&mut self, y: usize, x: usize) {
        if self.map.something_at_index(y, x) &&
           self.map.state_at_index(y, x) == State::Free {
               match self.map.material_at_index(y, x) {
                   Material::Sand => self.handle_sand(y, x),
                   Material::Water => self.handle_water(y, x),
                   Material::Stone => ()
                }
           }
    }

    fn handle_water(&mut self, y: usize, x: usize) {
        if  y < (self.buffer_height - 1) {
            if  !self.map.something_at_index(y + 1, x) {
                self.move_material(y, x, y + 1, x);
                self.map.change_state_at_index(y + 1, x, State::Set);
            } else if self.map.something_at_index(y + 1, x) &&
                x > 1 && x < (self.buffer_width-2) {
                self.move_water_sideways(y, x);
            }
        } else {
            self.remove_material(y, x);
        }
    }

    fn move_water_sideways(&mut self, y: usize, x: usize) {
        if rand::random::<bool>() {
            if !self.map.something_at_index(y, x + 2) {
                self.move_material(y, x, y, x + 2);
                self.map.change_state_at_index(y, x + 2, State::Set);
            } else if !self.map.something_at_index(y + 1, x + 1) {
                self.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else if !self.map.something_at_index(y, x - 2) {
                self.move_material(y, x, y, x - 2);
                self.map.change_state_at_index(y, x - 2, State::Set);
            } else if !self.map.something_at_index(y, x - 1) {
                self.move_material(y, x, y, x - 1);
                self.map.change_state_at_index(y, x - 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x - 1) {
                self.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        } else {
            if !self.map.something_at_index(y, x - 2) {
                self.move_material(y, x, y, x - 2);
                self.map.change_state_at_index(y, x - 2, State::Set);
            } else if !self.map.something_at_index(y + 1, x - 1) {
                self.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else if !self.map.something_at_index(y, x + 2) {
                self.move_material(y, x, y, x + 2);
                self.map.change_state_at_index(y, x + 2, State::Set);
            } else if !self.map.something_at_index(y, x + 1) {
                self.move_material(y, x, y, x + 1);
                self.map.change_state_at_index(y, x + 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x + 1) {
                self.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        }
    }


    fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        self.map.move_material(yfrom, xfrom, yto, xto);
        let offset_from = yfrom * window::SCREEN_WIDTH * 3 + (xfrom * 3);
        let offset_to = yto * window::SCREEN_WIDTH * 3 + (xto * 3);

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

    fn remove_material(&mut self, y: usize, x: usize) {
        self.map.remove_at_position(y, x);
        let offset = y * window::SCREEN_WIDTH*3 + x * 3;

        self.pixel_buffer[offset + 0] = 0;
        self.pixel_buffer[offset + 1] = 0;
        self.pixel_buffer[offset + 2] = 0;
    }

    fn handle_sand(&mut self, y: usize, x: usize) {
        if y < (self.buffer_height - 1) &&
           self.map.something_at_index(y + 1, x) &&
           x > 0 && x < (self.buffer_width-1) {
            self.try_move_side_down(y, x);
        } else if  y < (self.buffer_height -1) &&
            !self.map.something_at_index(y + 1, x) {
            self.move_material(y, x, y + 1, x);
            self.map.change_state_at_index(y + 1, x, State::Set);
        } else {
            self.remove_material(y, x);
        }
    }

    fn try_move_side_down(&mut self,y: usize,x: usize){
        if rand::random::<bool>() {
            if !self.map.something_at_index(y, x + 1) {
                self.move_material(y, x, y, x + 1);
                self.map.change_state_at_index(y, x + 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x + 1) {
                self.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        } else {
            if !self.map.something_at_index(y, x - 1) {
                self.move_material(y, x, y, x - 1);
                self.map.change_state_at_index(y, x - 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x - 1) {
                self.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        }
    }

    fn block_to_right(&self, y: usize, x: usize, count: usize) -> bool {
        let mut ret = true;
        for i in 0..count {
            ret = ret && self.map.something_at_index(y, x + i);
        }
        ret
    }

    fn block_to_left(&self, y: usize, x: usize, count: usize) -> bool {
        let mut ret = true;
        for i in 0..count {
            ret = ret && self.map.something_at_index(y, x - i);
        }
        ret
    }

    fn block_above(&self, y: usize, x: usize, count: usize) -> bool {
        let mut ret = true;
        for i in 0..count {
            ret = ret && self.map.something_at_index(y + i, x);
        }
        ret
    }

    fn bulk_move(&mut self, y: usize, x: usize) {
        if y > 4 && y < (self.buffer_height - 4) {
            if self.block_above(y, x, 4) {
                if self.block_to_right(y, x, 4) {
                    if !self.map.something_at_index(y, x - 3 ) {
                        self.move_material(y, x, y, x - 3);
                        self.map.change_state_at_index(y, x - 3, State::Set);
                    } else {
                        self.map.change_state_at_index(y, x, State::Set);
                    }
                } else if self.block_to_left(y, x, 4) {
                    if !self.map.something_at_index(y, x + 3 ) {
                        self.move_material(y, x, y, x + 3);
                        self.map.change_state_at_index(y, x + 3, State::Set);
                    } else {
                        self.map.change_state_at_index(y, x, State::Set);
                    }
                }
            }
        } else {
            self.map.change_state_at_index(y, x, State::Set);
        }
    }
}

