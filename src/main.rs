//#![windows_subsystem = "windows"]
extern crate sdl2; 
mod snake;
mod engine;

use engine::Engine;
use sdl2::rect::Point;

pub fn main() {

    let scale: usize = 5;
    let unit: usize = 15;
    let gamespeed: f32 = 12.00;
    let width = (scale * 144) as usize;
    let height = (scale * 159) as usize;
    let bounds = (Point::new(30, 225), Point::new((width-30).try_into().unwrap(), 630));
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
    
    let mut engine = Engine::new(width, height, unit.try_into().unwrap(), gamespeed, bounds, canvas, event_pump);
    
    engine.start();
}