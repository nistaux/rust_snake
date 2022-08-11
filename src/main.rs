//#![windows_subsystem = "windows"]
extern crate sdl2; 
mod snake;
mod engine;

use engine::GameEventCode;
use engine::Engine;


pub fn main() {

    let scale: usize = 5;
    let unit: usize = 15;
    let width = (scale * 240) as usize;
    let height = (scale * 135) as usize;
    let title = "Rust the Snake";

    // Setup SDL2 Stuffs
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, width.try_into().unwrap(), height.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    
    let mut engine = Engine::new(width, height, unit.try_into().unwrap(), 3, canvas, event_pump);
    
    while engine.running {
        
        engine.draw();
        engine.tick();

        for event in engine.get_events(){
            match event {
                Some(GameEventCode::Quit) => {
                    println!("Game Quiting");
                    engine.stop();
                },
                None => {
                    //println!("Non quit event happening");
                }
            }
        }
    }
}