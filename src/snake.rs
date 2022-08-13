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
    pub fn new(unit: i32, bounds: (Point, Point)) -> Snake {
        let (tail, body, head) = (
            Point::new(unit*39, unit*28),
            Point::new(unit*40, unit*28),
            Point::new(unit*41, unit*28),
        );
        Snake {direction: Direction::Left, body: vec![head, body, tail], food: Snake::new_food(unit, bounds)}
    }

    pub fn new_food(unit: i32, bounds: (Point, Point)) -> Point {
        let mut rng = rand::thread_rng();
        let (tlbound, brbound) = bounds; 
        let (x, y): (i32, i32) = (rng.gen_range((tlbound.x/unit)..(brbound.x/unit)), rng.gen_range((tlbound.y/unit)..brbound.y/unit));
        Point::new(x*unit, y*unit)
        
    }
    
    pub fn slither(&mut self, unit: i32, bounds: (Point, Point)) -> Option<GameState> {
        let (_, body) = self.body.split_first().unwrap();
        let point = self.body.last().unwrap().to_owned();

        let (tlbound, brbound) = bounds;
        match self.direction {
            Direction::Up => {
                if point.y() <=  tlbound.y{
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x(), point.y()-unit));
                    None
                }
            },
            Direction::Down => {
                let boundary = brbound.y - unit;
                if point.y() >= boundary {
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x(), point.y()+unit));
                    None
                }
            },
            Direction::Left => {
                if point.x() <= tlbound.x {
                    Some(GameState::EndGame)
                }else{
                    self.body = body.to_owned();
                    self.body.push(Point::new(point.x()-unit, point.y()));
                    None
                }
            },
            Direction::Right => {
                let boundary = brbound.x - unit;
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

    pub fn eat(&mut self, unit: i32, bounds: (Point, Point)) {
        self.food = Snake::new_food(unit, bounds);
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