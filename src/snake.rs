use rand::Rng;
use sdl2::rect::Point;

pub struct SnakeGame {
    pub width: i32,
    pub height: i32,
    gamespeed: u8,
    pub running: bool,
    food_loc: Point,
}

impl SnakeGame {
    pub fn new(width: i32, height: i32, gamespeed: u8) -> SnakeGame {
        SnakeGame {width, height, gamespeed, running: true, food_loc: Point::new(width - (width/4), height - (height/2))}
    }

    pub fn _change_gamespeed(&mut self, updated_gamespeed: u8) {
        if(!self.running){
            self.gamespeed = updated_gamespeed;
        }
    }

    pub fn _new_food(&mut self) {
        let mut rng = rand::thread_rng();
        let (x, y): (i32, i32) = (rng.gen_range(1..self.width), rng.gen_range(1..self.height));
        self.food_loc = Point::new(x, y);
        
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}

pub enum GameState {
    MainMenu,
    EndGame,
    Snake,
}