//extern crate sdl2;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use crate::snake::Snake;

pub struct Engine {
    // Self Made Things
    pub width: usize,
    pub height: usize,
    pub running: bool,
    pub unit: u8,
    pub gamespeed: u8,
    snake: Snake,

    // SDL Things
    canvas: Canvas<Window>,
    event_pump: EventPump,
    
}

impl Engine {
    pub fn new(width: usize, height: usize, unit: u8, gamespeed: u8, canvas: Canvas<Window>, event_pump: EventPump) -> Engine {
        Engine {
            width,
            height,
            running: true,
            unit,
            gamespeed,
            snake: Snake::new(unit.try_into().unwrap()),
            canvas,
            event_pump,
        }
    }

    pub fn get_events(&mut self) -> Vec<Option<GameEventCode>> {
        let mut events: Vec<Option<GameEventCode>> = vec![];
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => events.push(Some(GameEventCode::Quit)),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => events.push(Some(GameEventCode::Quit)),
                _ => events.push(None)
            }
        }
        events
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 180, 255));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        for i in (1..(self.width/(self.unit as usize))){
            let unit: usize = self.unit.try_into().unwrap();
            let start: Point = Point::new((i*unit).try_into().unwrap(), 0);
            let end: Point = Point::new((i*unit).try_into().unwrap(), self.height.try_into().unwrap());
            self.canvas.draw_line(start, end).unwrap();
        }

        for i in (1..(self.height/(self.unit as usize))){
            let unit: usize = self.unit.try_into().unwrap();
            let start: Point = Point::new(0, (i*unit).try_into().unwrap());
            let end: Point = Point::new(self.width.try_into().unwrap(), (i*unit).try_into().unwrap());
            self.canvas.draw_line(start, end).unwrap();
        }

        self.canvas.present();
    }

    pub fn tick(&self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn start_game() {
        todo!()
    }
}

pub enum GameEventCode {
    Quit,
}