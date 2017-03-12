use pixel_buffer;
use material;
use sdl2;

pub struct SimulationEngine {
    pixel_buffer: pixel_buffer::PixelBuffer,
    buffer_width: usize,
    buffer_height: usize
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize) -> SimulationEngine {
        SimulationEngine {
            pixel_buffer: pixel_buffer::new(width, height),
            buffer_width: width,
            buffer_height: height
        }
    }

    pub fn rgb_index(&self, x: usize, y: usize)-> &material::RGB {
        (&self.pixel_buffer)[y][x].rgb()
    }

    pub fn update(&self, texture: &mut sdl2::render::Texture) {
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..self.buffer_height {
                for x in 0..self.buffer_width {
                    let offset = y*pitch + x*3;
                    buffer[offset + 0] = self.rgb_index(x, y).red as u8;
                    buffer[offset + 1] = self.rgb_index(x, y).green as u8;
                    buffer[offset + 2] = self.rgb_index(x, y).blue as u8;
                }
            }
        }).unwrap();
    }
}
