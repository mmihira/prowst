use SimulationEngine;
use material::State;
use time;
use rand;
use rand::distributions::{IndependentSample, Range};

pub trait UpdateCellPositions {
    fn update_cell_positions(&mut self);
    fn try_move_side_down(&mut self, y: usize, x: usize);
    fn update_material(&mut self, y: usize, x: usize);
}

impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self) {
        self.map.reset_states();

        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                if self.map.something_at_index(y, x) &&
                   self.map.state_at_index(y, x) == State::Free &&
                   y < (self.buffer_height - 1) {
                        self.update_material(y, x);
                    }
            }
        }
    }

    fn update_material(&mut self, y: usize, x: usize) {
        if self.map.something_at_index(y + 1, x) &&
           x > 0 && x < self.buffer_width-1 {
            self.try_move_side_down(y, x);
        } else if !self.map.something_at_index(y + 1, x) {
            self.map.move_material(y, x, y + 1, x);
            self.map.change_state_at_index(y + 1, x, State::Set);
        } else {
            self.map.change_state_at_index(y, x, State::Set);
        }
    }

    fn try_move_side_down(&mut self,y: usize,x: usize){
        if rand::random::<bool>() {
            if !self.map.something_at_index(y + 1, x + 1) {
                self.map.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x - 1) {
                self.map.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        }else {
            if !self.map.something_at_index(y + 1, x - 1) {
                self.map.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x + 1) {
                self.map.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        }
    }
}

