extern crate sdl2;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn check_events(mut event_pump: &mut EventPump) -> Vec<Option<GameEventCode>> {
    let mut events: Vec<Option<GameEventCode>> = vec![];
    for event in event_pump.poll_iter() {
        
        match event {
            Event::Quit {..} => events.push(Some(GameEventCode::Quit)),
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => events.push(Some(GameEventCode::Quit)),
            _ => events.push(None)
        }
    }
    events
}

pub fn tick() {
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //println!("ticking");
}

pub enum GameEventCode {
    Quit,
}