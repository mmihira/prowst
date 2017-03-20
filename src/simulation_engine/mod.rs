pub mod trait_update_cell_positions;
pub mod trait_update_pixel_buffer;

use pixel_buffer;
use material::Material;
use material::RGB;
use sdl2;
use sdl2::event::Event;
use time;
use std::mem;
use std::vec;

pub struct SimulationEngine {
    pixel_buffer: pixel_buffer::PixelBuffer,
    buffer_width: usize,
    buffer_height: usize,
    time_at_last_update: time::SteadyTime,
    cells_to_update: Vec<Loc>,
    mouse_button_down: bool,
    selected_material: Material
}

trait UpdateCellPositions {
    fn update_cell_positions(&mut self);
}

trait UpdatePixelBuffer {
    fn update_pixel_buffer(&mut self);
}

#[derive(Debug)]
pub struct Loc {
    prev: (usize, usize),
    curr: (usize, usize),
    state: State
}

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Calc,
    Set,
    Dead
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize) -> SimulationEngine {
        SimulationEngine {
            pixel_buffer: pixel_buffer::new(width, height),
            buffer_width: width,
            buffer_height: height,
            time_at_last_update: time::SteadyTime::now(),
            cells_to_update: vec![Loc{curr: (10, 10), prev: (10, 10), state: State::Calc}],
            mouse_button_down: false,
            selected_material: Material::sand()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match *event {
            Event::KeyUp {keycode, ..}
                if keycode.unwrap() == sdl2::keyboard::Keycode::K => {
                    self.selected_material = Material::stone();
            },
            Event::KeyUp {keycode, ..}
                if keycode.unwrap() == sdl2::keyboard::Keycode::S => {
                    self.selected_material = Material::sand();
            },
            Event::MouseButtonDown {..} => {
                self.mouse_button_down = true;
            },
            Event::MouseButtonUp {..} => {
                self.mouse_button_down = false
            },
            Event::MouseMotion {x, y, ..} => {
                if self.mouse_button_down {
                    self.add_to_map(x as usize, y as usize);
                }
            },
            _ => {}
        }
    }

    pub fn rgb_index(&self, x: usize, y: usize)-> &RGB {
        (&self.pixel_buffer)[y][x].contents.rgb()
    }

    pub fn add_to_map(&mut self, x: usize, y: usize) {
        let ref mut row = self.pixel_buffer[y];
        row[x].contents = self.selected_material.clone();
        self.cells_to_update.push( Loc { curr: (y, x), prev: (y, x), state: State::Calc } )
    }

    pub fn add_sand(&mut self, x: usize, y: usize) {
        let row = &mut self.pixel_buffer[y];
        row[x].contents = Material::sand();
        self.cells_to_update.push( Loc { curr: (y, x), prev: (y, x), state: State::Calc } )
    }

    fn clean_dead(&mut self) {
        self.cells_to_update.retain(|ref x| x.state != State::Dead);
    }


    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previous_update = self.time_at_last_update;
        if time::SteadyTime::now() - previous_update > time::Duration::milliseconds(50) {
            self.time_at_last_update = time::SteadyTime::now();
            self.update_cell_positions();
            self.update_pixel_buffer();
            self.update_texture(texture);
        }
    }

    fn update_texture(&mut self, texture: &mut sdl2::render::Texture) {
        let mut z: [u8; 800*600*3] = [0; 800*600*3];
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                let offset = y*2400+ x*3;
                z[offset + 0] = self.rgb_index(x, y).red as u8;
                z[offset + 1] = self.rgb_index(x, y).green as u8;
                z[offset + 2] = self.rgb_index(x, y).blue as u8;
            }
        }
        texture.update(None,&z,2400).unwrap();
    }
}
