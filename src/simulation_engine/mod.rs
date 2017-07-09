use std::mem;
use std::vec;
use std::collections::HashMap;

use sdl2;
use sdl2::event::Event;

use time;
use uuid::Uuid;

use material::Material;
use material::RGB;
use material::State;
use material_map::MaterialMap;

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
                    self.map.add_material(x as usize, y as usize, self.selected_material.clone());
                }
            },
            _ => {}
        }
    }

    fn rgb_index(&self, x: usize, y: usize) -> RGB {
        match self.map.mat_map[y][x].contents {
            Some(u) => self.map.rgb_of_uuid(u),
            None => RGB{ red: 0, green: 0, blue: 0}
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
        let mut z: [u8; 800*600*3] = [0; 800*600*3];
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                let offset = y * 2400 + x * 3;
                z[offset + 0] = self.rgb_index(x, y).red as u8;
                z[offset + 1] = self.rgb_index(x, y).green as u8;
                z[offset + 2] = self.rgb_index(x, y).blue as u8;
            }
        }
        texture.update(None,&z,2400).unwrap();
    }
}
