extern crate sdl2;
extern crate time;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::{Surface,SurfaceRef};

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

    // let mut renderer = window.renderer().accelerated().build().unwrap();
    // let mut texture = renderer.create_texture_streaming( PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    let mut surface = Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::RGB24).unwrap();
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
        simulation_engine.update(&mut surface);
        // get the surface used by our window
        let screen = window.surface(&event_pump).unwrap();
        // copy our surface unto the window's surface
        //copy our surface unto the window's surface
        unsafe {
            // this is somewhat ugly, but in the current state
            // there is no easy SurfaceRef -> Surface conversion
            let _ = surface.blit(None, SurfaceRef::from_ll_mut(screen.raw()), None);
        }
        {
            // update the window to display the changed surface
            match window.update_surface() {
                Ok(_) => {},
                Err(err) => panic!("failed to update window surface: {}", err)
            }
        }
    }
}
