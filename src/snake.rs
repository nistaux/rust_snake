use rand::Rng;
use sdl2::rect::Point;

pub struct Snake {
    direction: Direction,
    body: Vec<Point>,
    food: Point,
}

enum Direction { Up, Down, Left, Right }


impl Snake {
    pub fn new(unit: i32) -> Snake {
        let (head, tail) = (
            Point::new((unit*(10*16)), (unit*(10*9))),
            Point::new((unit*(11*16)), (unit*(10*9)))
        );
        Snake {direction: Direction::Left, body: vec![head, tail], food: Point::new((unit*(5*16)), (unit*(10*9)))}
    }

    pub fn _new_food(&mut self) {
        let mut rng = rand::thread_rng();
        let (x, y): (i32, i32) = (rng.gen_range(1..1200), rng.gen_range(1..675));
        self.food = Point::new(x, y);
        
    }
}

pub enum GameState {
    MainMenu,
    EndGame,
    Snake,
}