use SimulationEngine;
use simulation_engine::UpdatePixelBuffer;
use simulation_engine::State;
use simulation_engine::Loc;

use material::Material;

trait UpdatePixelBufferPrivate {
    fn set_content_at_curr(&mut self,index_of_loc: usize, new_contents: &Material);

    fn set_content_at_prev(&mut self,index_of_loc: usize, new_contents: &Material);

    fn contents_at_prev(&mut self, index_of_loc: usize) -> Material;
}

impl UpdatePixelBufferPrivate for SimulationEngine {
    fn set_content_at_curr(&mut self,index_of_loc: usize, new_contents: &Material) {
        let Loc { curr: (y, x), .. } = self.cells_to_update[index_of_loc];
        self.pixel_buffer[y][x].contents = new_contents.clone();
    }

    fn set_content_at_prev(&mut self,index_of_loc: usize, new_contents: &Material) {
        let Loc { prev: (y, x), .. } = self.cells_to_update[index_of_loc];
        self.pixel_buffer[y][x].contents = new_contents.clone();
    }

    fn contents_at_prev(&mut self, index_of_loc: usize) -> Material {
        let Loc { prev: (y, x), .. } = self.cells_to_update[index_of_loc];
        self.pixel_buffer[y][x].contents.clone()
    }
}

impl UpdatePixelBuffer for SimulationEngine {
    fn update_pixel_buffer(&mut self) {
        let z = self.cells_to_update.len();
        for index_of_cell in 0..z {
            match self.cells_to_update[index_of_cell].state.clone() {
                State::Calc => {
                    let old_contents = (self).contents_at_prev(index_of_cell);
                    self.set_content_at_curr(index_of_cell, &old_contents);
                    self.set_content_at_prev(index_of_cell, &Material::default());
                },
                State::Dead => {
                    self.set_content_at_prev(index_of_cell, &Material::default());
                },
                _ => {}
            }
        }
    }
}
