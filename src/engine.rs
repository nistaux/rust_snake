//extern crate sdl2;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::LoadTexture;

use crate::snake::{Snake, GameState, Direction, PartType};

pub struct Engine {
    // Self Made Things
    pub width: usize,
    pub height: usize,
    pub running: bool,
    pub unit: u8,
    pub gamespeed: f64,
    snake: Snake,

    // SDL Things
    canvas: Canvas<Window>,
    event_pump: EventPump,    
}

impl Engine {
    pub fn new(width: usize, height: usize, unit: u8, gamespeed: f64, canvas: Canvas<Window>, event_pump: EventPump) -> Engine {
        Engine {
            width,
            height,
            running: true,
            unit,
            gamespeed,
            snake: Snake::new(unit.try_into().unwrap(), width.try_into().unwrap(), height.try_into().unwrap()),
            canvas,
            event_pump,
        }
    }

    pub fn get_events(&mut self) -> Vec<Option<GameEventCode>> {
        let mut events: Vec<Option<GameEventCode>> = vec![];
        for event in self.event_pump.poll_iter() {
            match event {
                // clicks X at top right
                Event::Quit {..} => events.push(Some(GameEventCode::Quit)),
                // presses esc
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => events.push(Some(GameEventCode::Quit)),
                // presses W
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    if self.snake.direction != Direction::Down{
                        self.snake.direction = Direction::Up;
                    }
                    events.push(None);
                },
                // presses A
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    if self.snake.direction != Direction::Right {
                        self.snake.direction = Direction::Left;
                    }
                    events.push(None);
                },
                // presses S
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if self.snake.direction != Direction::Up {
                        self.snake.direction = Direction::Down;
                    }
                    events.push(None);
                },
                // presses D
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if self.snake.direction != Direction::Left {
                        self.snake.direction = Direction::Right;
                    }
                    events.push(None);
                },
                // presses space
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.gamespeed = self.gamespeed + 20.0;
                    events.push(None);
                },
                _ => events.push(None)
            }
        }
        events
    }

    pub fn draw(&mut self, snake_texture: &Texture, map_texture: &Texture) {
        // Setting default background color
        self.canvas.set_draw_color(Color::RGB(212, 206, 125));
        //clearing the canvas
        self.canvas.clear();

        // map cutouts of the parts
        let head_rect = Rect::new(0, 0, 15, 15);
        let tail_rect = Rect::new(30, 0, 15, 15);
        let map = Rect::new(0, 0, self.width as u32, self.height as u32);

        // Getting head and tail first
        let tail = self.snake.body.first().unwrap();
        let (head, body) = self.snake.body.split_last().unwrap();

        // Setting background
        self.canvas.copy(&map_texture, map, None).unwrap();

        // Creating grid just to help for now
        self.canvas.set_draw_color(Color::RGB(150, 150, 90));
        for i in 1..(self.width/(self.unit as usize)){
            let unit: usize = self.unit.try_into().unwrap();
            let start: Point = Point::new((i*unit).try_into().unwrap(), 0);
            let end: Point = Point::new((i*unit).try_into().unwrap(), self.height.try_into().unwrap());
            self.canvas.draw_line(start, end).unwrap();
        }
        for i in 1..(self.height/(self.unit as usize)){
            let unit: usize = self.unit.try_into().unwrap();
            let start: Point = Point::new(0, (i*unit).try_into().unwrap());
            let end: Point = Point::new(self.width.try_into().unwrap(), (i*unit).try_into().unwrap());
            self.canvas.draw_line(start, end).unwrap();
        }
        self.canvas.set_draw_color(Color::RGB(255, 50, 50));
        self.canvas.fill_rect(Rect::new(self.snake.food.x(), self.snake.food.y(), u32::from(self.unit), u32::from(self.unit))).unwrap();

        // loop through body and render that
        let mut i: usize = 0;
        for part in body.to_owned() {
            let src: Rect;
            match self.snake.get_part_direction(part, i) {
                PartType::LRBody =>{
                    src = Rect::new(15, 0, 15, 15);
                    self.canvas.copy(
                        &snake_texture, 
                        src, 
                        Rect::new(part.x(), part.y(), 15, 15),
                    ).unwrap();
                },
                PartType::UDBody => {
                    src = Rect::new(15, 0, 15, 15);
                    self.canvas.copy_ex(
                        &snake_texture, 
                        src, 
                        Rect::new(part.x()+i32::from(self.unit), part.y(), 15, 15), 
                        90.00, 
                        Point::new(0, 0),
                        false,
                        false
                    ).unwrap();
                },
                PartType::TLCorner => {
                    src = Rect::new(0, 15, 15, 15);
                    self.canvas.copy(
                        &snake_texture, 
                        src, 
                        Rect::new(part.x(), part.y(), 15, 15),
                    ).unwrap();
                },
                PartType::TRCorner => {
                    src = Rect::new(0, 15, 15, 15);
                    self.canvas.copy_ex(
                        &snake_texture, 
                        src, 
                        Rect::new(part.x(), part.y(), 15, 15), 
                        0.00, 
                        Point::new(0, 0),
                        true,
                        false
                    ).unwrap();
                },
                PartType::BLCorner => {
                    src = Rect::new(15, 15, 15, 15);
                    self.canvas.copy(
                        &snake_texture, 
                        src, 
                        Rect::new(part.x(), part.y(), 15, 15),
                    ).unwrap();
                },
                PartType::BRCorner => {
                    src = Rect::new(15, 15, 15, 15);
                    self.canvas.copy_ex(
                        &snake_texture, 
                        src, 
                        Rect::new(part.x(), part.y(), 15, 15), 
                        0.00, 
                        Point::new(0, 0),
                        true,
                        false
                    ).unwrap();
                },
                PartType::UTail => {
                    self.canvas.copy_ex(
                        &snake_texture, 
                        tail_rect, 
                        Rect::new(tail.x()+i32::from(self.unit), tail.y(), 15, 15), 
                        90.00, 
                        Point::new(0, 0),
                        false,
                        false
                    ).unwrap();
                },
                PartType::DTail => {
                    self.canvas.copy_ex(
                        &snake_texture, 
                        tail_rect, 
                        Rect::new(head.x(), head.y(), 15, 15), 
                        -90.00, 
                        Point::new(0, 0),
                        false,
                        false
                    ).unwrap();
                },
                PartType::RTail => {
                    self.canvas.copy_ex(
                        &snake_texture, 
                        tail_rect, 
                        Rect::new(tail.x(), tail.y(), 15, 15), 
                        0.00, 
                        Point::new(0, 0),
                        true,
                        false
                    ).unwrap();
                },
                PartType::LTail => {
                    self.canvas.copy(
                        &snake_texture, 
                        tail_rect, 
                        Rect::new(tail.x(), tail.y(), 15, 15)).unwrap();
                },
            }
            i = i+1;
            //self.canvas.copy(&snake_texture, src, Rect::new(part.x(), part.y(), 15, 15)).unwrap();
        }

        // Drawing head based on current direction
        match self.snake.direction {
            Direction::Up => {
                self.canvas.copy_ex(
                    &snake_texture, 
                    head_rect, 
                    Rect::new(head.x()+i32::from(self.unit), head.y(), 15, 15), 
                    90.00, 
                    Point::new(0, 0),
                    false,
                    false
                ).unwrap();
            },
            Direction::Down => {
                self.canvas.copy_ex(
                    &snake_texture, 
                    head_rect, 
                    Rect::new(head.x(), head.y()+i32::from(self.unit), 15, 15), 
                    -90.00, 
                    Point::new(0, 0),
                    false,
                    false
                ).unwrap();
            },
            Direction::Left => {
                self.canvas.copy(
                    &snake_texture, 
                    head_rect, 
                    Rect::new(head.x(), head.y(), 15, 15)).unwrap();
            },
            Direction::Right => {
                self.canvas.copy_ex(
                    &snake_texture, 
                    head_rect, 
                    Rect::new(head.x(), head.y(), 15, 15), 
                    0.00, 
                    Point::new(0, 0),
                    true,
                    false
                ).unwrap();
            },
        }

        // Finally presenting the canvas after all
        self.canvas.present();
    }

    pub fn tick(&mut self) {
        // setting tickrate via gamespeed
        let tickrate: f64 = 1.00 / self.gamespeed;
        std::thread::sleep(Duration::from_secs_f64(tickrate));

        // moving snek if not hitting bounds
        match self.snake.slither(i32::from(self.unit,), self.width, self.height) {
            Some(GameState::EndGame) => {
                //self.stop()
                match self.snake.direction {
                    Direction::Up => println!("Hitting Top Edge!"),
                    Direction::Down => println!("Hitting Bottom Edge!"),
                    Direction::Left => println!("Hitting Left Edge!"),
                    Direction::Right => println!("Hitting Right Edge!"),
                }
            },
            _ => {}
        }
        let head = self.snake.body.last().unwrap().to_owned();
        if (head.x(), head.y()) == (self.snake.food.x(), self.snake.food.y()) {
            self.snake.eat(i32::from(self.unit), self.width as i32, self.height as i32);
            self.gamespeed = self.gamespeed + 0.40;
            println!("Tickrate: {} - Gamespeed: {}", tickrate, self.gamespeed);
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn _start_game() {
        todo!()
    }

    pub fn start(&mut self){
        let texture_creator = self.canvas.texture_creator();
        let map_texture = texture_creator.load_texture(".\\imgs\\floor.png").unwrap();
        let snake_texture = texture_creator.load_texture(".\\imgs\\snake_map.png").unwrap();
        while self.running {
            
            self.draw(&snake_texture, &map_texture);
            self.tick();

            for event in self.get_events(){
                match event {
                    Some(GameEventCode::Quit) => {
                        println!("Game Quiting");
                        self.stop();
                    },
                    None => {
                        //println!("Non quit event happening");
                    }
                }
            }
        }
    }
}

pub enum GameEventCode {
    Quit,
}