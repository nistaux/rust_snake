use rand::Rng;
use sdl2::rect::Point;

pub struct Snake {
    pub direction: Direction,
    pub body: Vec<Point>,
    pub food: Point,
}

#[derive(PartialEq)]
pub enum Direction { Up, Down, Left, Right }


impl Snake {
    pub fn new(unit: i32, width: i32, height: i32) -> Snake {
        let (tail, head) = (
            Point::new(unit*70, unit*21),
            Point::new(unit*71, unit*21)
        );
        Snake {direction: Direction::Left, body: vec![head, tail], food: Snake::new_food(unit, width, height)}
    }

    pub fn new_food(unit: i32, width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        let (x, y): (i32, i32) = (rng.gen_range(1..(width/unit)), rng.gen_range(1..height/unit));
        Point::new(x*unit, y*unit)
        
    }
    
    pub fn slither(&mut self, unit: i32, width: usize, height: usize) -> Option<GameState> {
        match self.direction {
            Direction::Up => {
                let (_, body) = self.body.split_first().unwrap();
                let point = self.body.last().unwrap().to_owned();
                if point.y() <= 0 {
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x(), point.y()-unit));
                    None
                }
            },
            Direction::Down => {
                let (_, body) = self.body.split_first().unwrap();
                let point = self.body.last().unwrap().to_owned();
                let boundary = (height as i32) - unit;
                if point.y() >= boundary {
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x(), point.y()+unit));
                    None
                }
            },
            Direction::Left => {
                let (_, body) = self.body.split_first().unwrap();
                let point = self.body.last().unwrap().to_owned();
                if point.x() <= 0 {
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x()-unit, point.y()));
                    None
                }
            },
            Direction::Right => {
                let (_, body) = self.body.split_first().unwrap();
                let point = self.body.last().unwrap().to_owned();
                let boundary = (width as i32) - unit;
                if point.x() >= boundary {
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x()+unit, point.y()));
                    None
                }
            },
        }
    }

    pub fn get_part_direction(&self, part: Point, i: usize) -> PartType {
        let fpoint = self.body[i+1];
        if i == 0 {
            if fpoint.x < part.x {
                PartType::LTail
            }else if fpoint.y < part.y {
                PartType::UTail
            }else if fpoint.x > part.x {
                PartType::RTail
            }else {
                PartType::DTail
            }
        }else {
            let bpoint = self.body[i-1];
            if fpoint.x < part.x {
                if      bpoint.x > part.x {PartType::LRBody}
                else if bpoint.y > part.y {PartType::TRCorner}
                else                      {PartType::BRCorner}
            }else if fpoint.y < part.y {
                if      bpoint.x > part.x {PartType::BLCorner}
                else if bpoint.y > part.y {PartType::UDBody}
                else                      {PartType::BRCorner}
            }else if fpoint.x > part.x {
                if      bpoint.x < part.x {PartType::LRBody}
                else if bpoint.y > part.y {PartType::TLCorner}
                else                      {PartType::BLCorner}
            // if fpoint.y > part.y
            }else  {
                if      bpoint.x > part.x {PartType::TLCorner}
                else if bpoint.y < part.y {PartType::UDBody}
                else                      {PartType::TRCorner}
            } 
        }
        
    }

    pub fn eat(&mut self, unit: i32, width: i32, height: i32) {
        self.food = Snake::new_food(unit, width, height);
        let current_tail = self.body.first().unwrap().to_owned();
        let mut new_body = vec![Point::new(current_tail.x()+unit, current_tail.y())];
        for part in &self.body {
            new_body.push(part.to_owned());
        }
        self.body = new_body;
    }
}

pub enum PartType {
    LRBody, 
    UDBody, 
    TLCorner, 
    TRCorner, 
    BLCorner, 
    BRCorner,
    UTail,
    DTail,
    RTail,
    LTail,
}

pub enum GameState {
    _MainMenu,
    EndGame,
    _Snake,
}