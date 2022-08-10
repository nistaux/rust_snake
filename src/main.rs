extern crate sdl2; 
mod snake;
mod engine;

use engine::GameEventCode;
use sdl2::pixels::Color;
use snake::SnakeGame;
 
pub fn main() {
    // Create SnakeGame instance
    let mut snakegame = SnakeGame::new(800, 600, 6);

    // Setup SDL2 Stuffs
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rust, the Snake", snakegame.width.try_into().unwrap(), snakegame.height.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // Setting default background color for now          
    canvas.set_draw_color(Color::RGB(0, 180, 255));
    canvas.clear();
    canvas.present();

    // Setting up event management
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Game Loop - Not sure how I want to do this
    while snakegame.running {
        
        // todo: make Engine struct and run everything from in it
        // instead of snakegame.running above, it could be engine.running
        // this should make all of this more simple

        engine::tick();

        for event in engine::check_events(&mut event_pump){
            match event {
                Some(GameEventCode::Quit) => {
                    println!("Game Quiting");
                    snakegame.stop();
                },
                None => {
                    //println!("Non quit event happening");
                }
            }
        }
    }
}