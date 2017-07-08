use SimulationEngine;

use simulation_engine::UpdateCellPositions;
use simulation_engine::State;
use time;

impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self) {
        // self.clean_dead();
        // for i in 0..self.cells_to_update.len() {
        //     let ref mut loc = self.cells_to_update[i as usize];
        //     match *&loc.state {
        //         State::Calc => {
        //             loc.prev = loc.curr.clone();
        //             loc.curr.0 = loc.curr.0 + 1;
        //             if loc.curr.0 == (self.buffer_height - 1) {
        //                 loc.state = State::Dead;
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    }
}

