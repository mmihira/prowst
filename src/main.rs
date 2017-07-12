extern crate sdl2;
extern crate time;
extern crate rand;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

mod material;
mod material_map;
mod cell;
mod simulation_engine;
mod brushes;
mod window;

use simulation_engine::SimulationEngine;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("prowst", window::SCREEN_WIDTH as u32, window::SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, window::SCREEN_WIDTH as u32, window::SCREEN_HEIGHT as u32).unwrap();
    let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    let mut simulation_engine = SimulationEngine::new(window::SCREEN_WIDTH as usize, window::SCREEN_HEIGHT as usize);
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
        renderer.copy(&texture, None, Some(Rect::new(0, 0, window::SCREEN_WIDTH as u32, window::SCREEN_HEIGHT as u32))).unwrap();
        renderer.present();
    }

}
