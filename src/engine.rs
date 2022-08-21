//extern crate sdl2;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::LoadTexture;

use crate::snake::{Snake, Direction, PartType};

pub struct Engine {
    // Self Made Things
    pub width: usize,
    pub height: usize,
    pub running: bool,
    pub unit: u8,
    pub gamespeed: f32,
    bounds: (Point, Point),
    snake: Snake,
    state: GameState,

    // SDL Things
    canvas: Canvas<Window>,
    event_pump: EventPump,  
}

impl Engine {
    pub fn new(width: usize, height: usize, unit: u8, gamespeed: f32, bounds: (Point, Point), canvas: Canvas<Window>, event_pump: EventPump) -> Engine {
        Engine {
            width,
            height,
            running: true,
            unit,
            gamespeed,
            bounds,
            snake: Snake::new(unit.try_into().unwrap(), bounds),
            state: GameState::MainMenu,
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
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    match self.state {
                        GameState::Game => {
                            events.push(None);
                        },
                        GameState::GameOver => {
                            self.state = GameState::MainMenu;
                            events.push(None);
                        },
                        GameState::MainMenu => {
                            self.state = GameState::Game;
                            events.push(Some(GameEventCode::Quit))
                        },

                    }
                },
                // presses W
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    if self.snake.get_head_direction() != Direction::Down{
                        self.snake.direction = Direction::Up;
                    }
                    events.push(None);
                },
                // presses A
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    if self.snake.get_head_direction() != Direction::Right {
                        self.snake.direction = Direction::Left;
                    }
                    events.push(None);
                },
                // presses S
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if self.snake.get_head_direction() != Direction::Up {
                        self.snake.direction = Direction::Down;
                    }
                    events.push(None);
                },
                // presses D
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if self.snake.get_head_direction() != Direction::Left {
                        self.snake.direction = Direction::Right;
                    }
                    events.push(None);
                },
                // presses space
                /*Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.gamespeed = self.gamespeed + 20.0;
                    events.push(None);
                },*/
                // presses Enter
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    match self.state {
                        GameState::Game => {
                            events.push(None);
                        },
                        GameState::GameOver => {
                            self.state = GameState::Game;
                            events.push(None);
                        },
                        GameState::MainMenu => {
                            self.state = GameState::Game;
                            events.push(Some(GameEventCode::StartGame))
                        },

                    }
                },
                _ => events.push(None)
                
            }
        }
        events
    }

    pub fn draw_art(&mut self, snake_texture: &Texture, map_texture: &Texture) {
        // Setting default background color
        self.canvas.set_draw_color(Color::RGB(212, 206, 125));
        //clearing the canvas
        self.canvas.clear();

        // map cutouts of the parts
        let head_rect = Rect::new(0, 0, 15, 15);
        let tail_rect = Rect::new(30, 0, 15, 15);
        let map = Rect::new(0, 200, self.width as u32, self.height as u32);

        // Getting head and tail first
        let tail = self.snake.body.first().unwrap();
        let (head, body) = self.snake.body.split_last().unwrap();

        // Setting background
        self.canvas.copy(&map_texture, map, None).unwrap();
        let (tlbound, brbound) = self.bounds;
        // Creating grid just to help for now
        self.canvas.set_draw_color(Color::RGB(43, 84, 25));
        for i in tlbound.x/(self.unit as i32)..(brbound.x/(self.unit as i32)){
            let unit: usize = self.unit.try_into().unwrap();
            let start: Point = Point::new((i*unit as i32).try_into().unwrap(), tlbound.y);
            let end: Point = Point::new((i*unit as i32).try_into().unwrap(), brbound.y);
            self.canvas.draw_line(start, end).unwrap();
        }
        for i in tlbound.y/self.unit as i32..(brbound.y/(self.unit as i32)){
            let unit: usize = self.unit.try_into().unwrap();
            let start: Point = Point::new(tlbound.x, (i*unit as i32).try_into().unwrap());
            let end: Point = Point::new(brbound.x, (i*unit as i32).try_into().unwrap());
            self.canvas.draw_line(start, end).unwrap();
        }

        // Draw Food (Lil Crab boi)
        self.canvas.copy(&snake_texture, Rect::new(30, 15, 15, 15), Rect::new(self.snake.food.x, self.snake.food.y, 15, 15)).unwrap();

        self.canvas.set_draw_color(Color::RGB(177, 199, 36));
        self.canvas.draw_rect(Rect::new(tlbound.x, tlbound.y, (brbound.x - tlbound.x) as u32, (brbound.y-tlbound.y) as u32)).unwrap();

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
                        Rect::new(tail.x(), tail.y()+i32::from(self.unit), 15, 15), 
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

        //self.canvas.te
        // Finally presenting the canvas after all
        
    }

    pub fn tick(&mut self) {
        // setting tickrate via gamespeed
        let tickrate: f32 = 1.00 / self.gamespeed;
        std::thread::sleep(Duration::from_secs_f32(tickrate));

        // moving snek if not hitting bounds
        match self.snake.slither(i32::from(self.unit,), self.bounds) {
            Some(GameState::GameOver) =>  self.state = GameState::GameOver,
            _ => {}
        }
        let head = self.snake.body.last().unwrap().to_owned();
        if (head.x(), head.y()) == (self.snake.food.x(), self.snake.food.y()) {
            self.snake.eat(i32::from(self.unit), self.bounds);
            self.gamespeed = self.gamespeed + 0.25;
            //println!("Tickrate: {} - Gamespeed: {}", tickrate, self.gamespeed);
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self){
        // Setting up textures
        let texture_creator = self.canvas.texture_creator();
        let map_texture = texture_creator.load_texture(".\\imgs\\floor.png").unwrap();
        let snake_texture = texture_creator.load_texture(".\\imgs\\snake_map_green.png").unwrap();
        let mainmenu_texture = texture_creator.load_texture(".\\imgs\\main_menu.png").unwrap();
        let map = Rect::new(0, 200, self.width as u32, self.height as u32);

        // Setting up fonts
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut title_font = ttf_context.load_font(".\\fonts\\lucon.ttf", 50).unwrap();
        title_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut ui_font = ttf_context.load_font(".\\fonts\\lucon.ttf", 27).unwrap();
        ui_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut option_font = ttf_context.load_font(".\\fonts\\lucon.ttf", 27).unwrap();
        option_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut gameover_font = ttf_context.load_font(".\\fonts\\lucon.ttf", 75).unwrap();
        gameover_font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
        let mut surface: Surface;
        let mut dst: Rect;

        while self.running {
            if self.state == GameState::MainMenu {
                self.canvas.clear();
    
                self.canvas.copy(&mainmenu_texture, map, None).unwrap();
    
                //16, 65, 179
                // Create Title Text
                let str = String::from("Rust the Snake");
                surface = title_font.render(&str).blended(Color::RGBA(150, 96, 36, 255)).unwrap();
                let title_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                let TextureQuery {width, height, ..} = title_text.query();
                dst = Rect::new(140,110, width, height);
                self.canvas.copy(&title_text, None, dst).unwrap();
    
                // Create Author Text
                let str = String::from("by Nistaux");
                surface = option_font.render(&str).blended(Color::RGBA(0, 138, 117, 255)).unwrap();
                let author_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                let TextureQuery {width, height, ..} = author_text.query();
                dst = Rect::new(450,165, width, height);
                self.canvas.copy(&author_text, None, dst).unwrap();
    
                // Creating box for text to go in
                self.canvas.set_draw_color(Color::RGB(230, 230, 230));
                self.canvas.fill_rect(Rect::new(180,395,357,38)).unwrap();
    
                // Create start game text (press enter to start)
                let str = String::from("Press Enter to start!");
                surface = option_font.render(&str).blended(Color::RGBA(16, 100, 179, 255)).unwrap();
                let option_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                let TextureQuery {width, height, ..} = option_text.query();
                dst = Rect::new(190,400, width, height);
                self.canvas.copy(&option_text, None, dst).unwrap();
    
                for event in self.get_events(){
                    match event {
                        Some(GameEventCode::Quit) => {
                            //println!("Game Quiting");
                            self.stop();
                            
                        },
                        Some(GameEventCode::StartGame) => {},
                        None => {
                            //println!("Non quit event happening");
                        }
                    }
                }
                self.canvas.present();
                std::thread::sleep(Duration::from_millis(33));
            }

            // starting game loop
            if self.state == GameState::Game || self.state == GameState::GameOver {

                // checking mouse and keyboard input
                for event in self.get_events(){
                    match event {
                        Some(GameEventCode::Quit) => {
                            //println!("Game Quiting");
                            self.stop();
                            
                        },
                        Some(GameEventCode::StartGame) => {},
                        None => {
                            //println!("Non quit event happening");
                        }
                    }
                }

                if self.state == GameState::GameOver {
                    // resetting core engine variables
                    self.snake = Snake::new(self.unit.try_into().unwrap(), self.bounds);
                    self.gamespeed = 12.00;

                    // Creating box for text to go in
                    self.canvas.set_draw_color(Color::RGB(20, 20, 80));
                    self.canvas.fill_rect(Rect::new(80,135,self.width as u32-160,225)).unwrap();

                    // Create GameOver Text
                    let str = String::from("Game Over!");
                    surface = gameover_font.render(&str).blended(Color::RGBA(220, 220, 255, 255)).unwrap();
                    let gameover_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = gameover_text.query();
                    dst = Rect::new(136,140, width, height);
                    self.canvas.copy(&gameover_text, None, dst).unwrap();

                    // Create Option(Main Menu or Try Again) Text
                    let str = String::from("Press ESC to return to Main Menu");
                    surface = option_font.render(&str).blended(Color::RGBA(50, 138, 117, 255)).unwrap();
                    let mainmenu_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = mainmenu_text.query();
                    dst = Rect::new(100,250, width, height);
                    self.canvas.copy(&mainmenu_text, None, dst).unwrap();

                    let str = String::from("Press ENTER to try again");
                    surface = option_font.render(&str).blended(Color::RGBA(50, 138, 117, 255)).unwrap();
                    let tryagain_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = tryagain_text.query();
                    dst = Rect::new(160,295, width, height);
                    self.canvas.copy(&tryagain_text, None, dst).unwrap();                    
                    
                }else {
                    // ticking game forward
                    self.tick();
                    // drawing textures to canvas
                    self.draw_art(&snake_texture, &map_texture);

                    // Creating box for text to go in
                    self.canvas.set_draw_color(Color::RGB(50, 105, 50));
                    self.canvas.fill_rect(Rect::new(15,self.height as i32-100,self.width as u32-30,90)).unwrap();

                    // Create Title Text
                    let str = String::from("Rust the Snake");
                    surface = title_font.render(&str).blended(Color::RGBA(150, 96, 36, 255)).unwrap();
                    let title_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = title_text.query();
                    dst = Rect::new(140,30, width, height);
                    self.canvas.copy(&title_text, None, dst).unwrap();

                    // Create Author Text
                    let str = String::from("by Nistaux");
                    surface = option_font.render(&str).blended(Color::RGBA(0, 138, 117, 255)).unwrap();
                    let author_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = author_text.query();
                    dst = Rect::new(450,75, width, height);
                    self.canvas.copy(&author_text, None, dst).unwrap();

                    // Create Gamespeed UI
                    let mut str = String::from("Game Speed: ");
                    if self.gamespeed.to_string().len() == 4 { 
                        str.push_str(&(self.gamespeed).to_string()[0..4].to_string()); 
                    } else if self.gamespeed.to_string().len() >= 5 { 
                        str.push_str(&(self.gamespeed).to_string()[0..5].to_string());
                    } else { 
                        str.push_str(&(self.gamespeed).to_string());
                    }
                    surface = ui_font.render(&str).blended(Color::RGBA(150, 210, 150, 255)).unwrap();
                    let gamespeed_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = gamespeed_text.query();
                    dst = Rect::new(390,self.height as i32-67, width, height);
                    self.canvas.copy(&gamespeed_text, None, dst).unwrap();

                    // Create Score UI
                    let mut str = String::from("Score: ");
                    str.push_str(&(self.snake.body.len()-3).to_string());
                    surface = ui_font.render(&str).blended(Color::RGBA(150, 210, 150, 255)).unwrap();
                    let gamespeed_text = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery {width, height, ..} = gamespeed_text.query();
                    dst = Rect::new(125,self.height as i32-67, width, height);
                    self.canvas.copy(&gamespeed_text, None, dst).unwrap();
                }
                self.canvas.present();
            }
        }
    }
}

#[derive(PartialEq)]
pub enum GameState {
    MainMenu,
    Game,
    GameOver,
}
pub enum GameEventCode {
    StartGame,
    Quit,
}