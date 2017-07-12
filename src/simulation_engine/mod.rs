use std::mem;
use std::vec;
use std::collections::HashMap;

use sdl2;
use sdl2::event::Event;

use time;

use material::Material;
use material::RGB;
use material::State;
use material_map::MaterialMap;
use brushes;
use window;

pub mod trait_update_cell_positions;
use simulation_engine::trait_update_cell_positions::UpdateCellPositions;

pub struct SimulationEngine {
    buffer_width: usize,
    buffer_height: usize,
    time_at_last_update: time::SteadyTime,
    map: MaterialMap,
    mouse_button_down: bool,
    selected_material: Material
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize) -> SimulationEngine {
        SimulationEngine {
            buffer_width: width,
            buffer_height: height,
            time_at_last_update: time::SteadyTime::now(),
            mouse_button_down: false,
            selected_material: Material::def_sand(),
            map: MaterialMap::new(width, height),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match *event {
            Event::KeyUp {keycode, ..}
                if keycode.unwrap() == sdl2::keyboard::Keycode::K => {
                    self.selected_material = Material::def_sand();
            },
            Event::KeyUp {keycode, ..}
                if keycode.unwrap() == sdl2::keyboard::Keycode::S => {
                    self.selected_material = Material::def_sand();
            },
            Event::MouseButtonDown {..} => {
                self.mouse_button_down = true;
            },
            Event::MouseButtonUp {..} => {
                self.mouse_button_down = false
            },
            Event::MouseMotion {x, y, ..} => {
                if self.mouse_button_down {
                    for cord in brushes::circle( 30.0, y, x, self.buffer_height, self.buffer_width, 0.8) {
                            self.map.add_material(cord.1 as usize,
                                                  cord.0 as usize,
                                                  self.selected_material.clone());
                    }
                }
            },
            _ => {}
        }
    }

    fn assign_rgb(&self,buff: &mut [u8], y: usize, x: usize) {
        let offset = y * window::SCREEN_WIDTH*3 + x * 3;
        if self.map.something_at_index(y,x) {
            self.map.copy_rgb_index(offset, buff, y, x);
        } else {
            buff[offset + 0] = 0;
            buff[offset + 1] = 0;
            buff[offset + 2] = 0;
        }
    }

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previous_update = self.time_at_last_update;
        if time::SteadyTime::now() - previous_update > time::Duration::milliseconds(10) {
            self.time_at_last_update = time::SteadyTime::now();
            self.update_cell_positions();
            self.update_texture(texture);
        }
    }

    fn update_texture(&mut self, texture: &mut sdl2::render::Texture) {
        let mut z: [u8; window::SCREEN_WIDTH * window::SCREEN_HEIGHT * 3] = [0; window::SCREEN_HEIGHT * window::SCREEN_WIDTH *3];
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                self.assign_rgb(&mut z, y, x);
            }
        }
        texture.update(None,&z,2400).unwrap();
    }
}
