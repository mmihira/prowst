use pixel_buffer;
use material;
use sdl2;
use time;
use std::mem;
use std::vec;

pub struct SimulationEngine {
    pixel_buffer: pixel_buffer::PixelBuffer,
    buffer_width: usize,
    buffer_height: usize,
    time_at_last_update: time::SteadyTime,
    cells_to_update: Vec<Loc>
}

#[derive(Debug)]
pub struct Loc {
    prev: (usize, usize),
    curr: (usize, usize)
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize) -> SimulationEngine {
        SimulationEngine {
            pixel_buffer: pixel_buffer::new(width, height),
            buffer_width: width,
            buffer_height: height,
            time_at_last_update: time::SteadyTime::now(),
            cells_to_update: vec![Loc{curr: (10, 10), prev: (0, 0)}]
        }
    }

    pub fn rgb_index(&self, x: usize, y: usize)-> &material::RGB {
        (&self.pixel_buffer)[y][x].rgb()
    }

    pub fn add_to_map(&self, x: usize, y: usize, k: material::Material ) {
    }

    pub fn add_sand(&mut self, x: usize, y: usize) {
        let row = &mut self.pixel_buffer[y];
        mem::replace(&mut row[x], material::Material::Sand(material::Sand::default()));
        self.cells_to_update.push( Loc { curr: (y, x), prev: (y, x) } )
    }

    fn update_cells(&mut self) {
        for cell in 0..self.cells_to_update.len() {
            let i = &mut self.cells_to_update[cell as usize];
            let state = self.pixel_buffer[i.curr.0][i.curr.1].state();
            match state {
                material::State::Free => {
                    let mut material = &mut self.pixel_buffer[i.curr.0][i.curr.1];
                    i.prev = i.curr;
                    i.curr.0 = i.curr.0 + 1;
                    if i.curr.0 == (self.buffer_height - 1) {
                        material.set_state(material::State::Dead);
                        println!("dead {:?}", material);
                    }
                },
                material::State::Dead => {

                },
                _ => {}
            }
        }
    }

    fn update_pixel_buffer(&mut self) {
        for i in &self.cells_to_update {
            let state = self.pixel_buffer[i.prev.0][i.prev.1].state();
            match state {
                material::State::Free => {
                    let k = self.pixel_buffer[i.prev.0][i.prev.1].clone();
                    mem::replace(&mut (&mut self.pixel_buffer)[i.curr.0][i.curr.1], k);
                    mem::replace(
                        &mut (&mut self.pixel_buffer)[i.prev.0][i.prev.1],
                        material::Material::Background(material::Background::default())
                        );
                },
                material::State::Dead => {
                },
                _ => {}
            }
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

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previousUpdate = self.time_at_last_update;

        if time::SteadyTime::now() - previousUpdate > time::Duration::milliseconds(50) {
            self.time_at_last_update = time::SteadyTime::now();
            self.update_cells();
            self.update_pixel_buffer();
            self.update_texture(texture);
        }
    }
}
