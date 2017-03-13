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

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        // let previousUpdate = self.time_at_last_update;
        // if time::SteadyTime::now() - previousUpdate > time::Duration::milliseconds(1) {
        if true {
            for ix in 0..self.cells_to_update.len() {
                let i = &mut self.cells_to_update[ix as usize];
                if *self.pixel_buffer[i.curr.0][i.curr.1].state() == material::State::Free {
                    i.prev = i.curr;
                    i.curr.0 = i.curr.0 + 1;
                }
            }
            // self.time_at_last_update = time::SteadyTime::now();
            for i in &self.cells_to_update {
                let k = self.pixel_buffer[i.prev.0][i.prev.1].clone();
                // println!("loc = {:?}", i );
                // println!("{:?}", k);
                mem::replace(&mut (&mut self.pixel_buffer)[i.curr.0][i.curr.1], k);
                mem::replace(
                    &mut (&mut self.pixel_buffer)[i.prev.0][i.prev.1],
                    material::Material::Background(material::Background::default())
                    );
            }
        }

        let z: [u8; 800*600*3] = [0; 800*600*3];
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                let offset = y*2400+ x*3;
                z[offset + 0] = self.rgb_index(x, y).red as u8;
                z[offset + 1] = self.rgb_index(x, y).green as u8;
                z[offset + 2] = self.rgb_index(x, y).blue as u8;
            }
        }

        texture.update(None,&z,2400).unwrap();

        // texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        //     for y in 0..self.buffer_height {
        //         for x in 0..self.buffer_width {
        //             println!("{:?}", pitch);
        //             let offset = y*pitch + x*3;
        //             buffer[offset + 0] = self.rgb_index(x, y).red as u8;
        //             buffer[offset + 1] = self.rgb_index(x, y).green as u8;
        //             buffer[offset + 2] = self.rgb_index(x, y).blue as u8;
        //         }
        //     }
        // }).unwrap()
    }
}
