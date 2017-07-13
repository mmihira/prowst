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
    selected_material: Material,
    pixel_buffer: [u8; window::SCREEN_WIDTH * window::SCREEN_HEIGHT * 3]
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
            pixel_buffer: [0; window::SCREEN_HEIGHT * window::SCREEN_WIDTH *3]
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
                        self.add_selected_to_map(cord.0 as usize, cord.1 as usize);
                    }
                }
            },
            _ => {}
        }
    }

    fn add_selected_to_map(&mut self, y: usize, x: usize) {
        if !self.map.something_at_index(y, x) {
            let offset = y * window::SCREEN_WIDTH*3 + x * 3;
            let rgb = self.selected_material.rgb();
            self.pixel_buffer[offset + 0] = rgb.red as u8;
            self.pixel_buffer[offset + 1] = rgb.green as u8;
            self.pixel_buffer[offset + 2] = rgb.blue as u8;
            self.map.add_material(y, x, self.selected_material.clone());
        }
    }

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previous_update = self.time_at_last_update;
        let time_elapsed = time::SteadyTime::now() - previous_update;
        if time_elapsed > time::Duration::milliseconds(10) {
            self.update_cell_positions(&time_elapsed);
            self.time_at_last_update = time::SteadyTime::now();
        }
        self.update_texture(texture);
    }

    fn update_texture(&mut self, texture: &mut sdl2::render::Texture) {
        texture.update(None, &self.pixel_buffer, 2400).unwrap();
    }
}
