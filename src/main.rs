extern crate sdl2;
extern crate time;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

mod material;
mod pixel_buffer;
mod simulation_engine;
use simulation_engine::SimulationEngine;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("prowst",   SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    let mut simulation_engine = SimulationEngine::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => { simulation_engine.handle_event(&event) }
            }
        }

        simulation_engine.update(&mut texture);
        renderer.clear();
        renderer.copy(&texture, None, Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT))).unwrap();
        renderer.present();
    }
}
