use SimulationEngine;
use simulation_engine::UpdateCellPositions;
use simulation_engine::State;
use simulation_engine::Loc;
use material::Material;
use time;

trait UpdateCellPositionPrivate {
    fn handle_sand_calc(&mut self, index_of_loc: usize);
    fn handle_stone_calc(&mut self, index_of_loc: usize);
    fn state_at_prev(&mut self, index_of_loc: usize) -> State;
    fn contents_at_curr(&mut self, index_of_loc: usize) -> Material;
}

impl UpdateCellPositionPrivate for SimulationEngine {
    fn handle_sand_calc(&mut self, index_of_loc: usize) {
        match self.state_at_prev(index_of_loc) {
            State::Calc => {
                let ref mut loc = self.cells_to_update[index_of_loc as usize];
                loc.prev = loc.curr.clone();
                loc.curr.0 = loc.curr.0 + 1;
                if loc.curr.0 == (self.buffer_height - 1) { loc.state = State::Dead; }
            },
            _ => {}
        }
    }

    fn handle_stone_calc(&mut self, index_of_loc: usize) {
        match self.state_at_prev(index_of_loc) {
            State::Calc => { },
            _ => {}
        }
    }

    fn state_at_prev(&mut self, index_of_loc: usize) -> State {
        let Loc { ref state, .. } = self.cells_to_update[index_of_loc];
        state.clone()
    }

    fn contents_at_curr(&mut self, index_of_loc: usize) -> Material {
        let Loc { curr: (y, x), .. } = self.cells_to_update[index_of_loc];
        self.pixel_buffer[y][x].contents.clone()
    }
}



impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self) {
        self.clean_dead();

        for i in 0..self.cells_to_update.len() {
            match self.contents_at_curr(i) {
                Material::Sand(_)   => { self.handle_sand_calc(i) },
                Material::Stone(_)  => { self.handle_stone_calc(i) },
                _ => {}
            }
        }
    }
}


