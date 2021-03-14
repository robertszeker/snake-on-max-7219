use super::point::Point;
use super::Direction;

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    pub tail: Vec<Point>,
}

impl Snake {
    pub fn init(tail: Vec<Point>) -> Snake {
        Snake {
            tail,
            direction: Direction::Left,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) -> () {
        self.direction = direction;
    }

    pub fn walk(&mut self) -> () {
        let mut head = Point { ..self.tail[0] };
        match self.direction {
            Direction::Left => head.move_left(),
            Direction::Right => head.move_right(),
            Direction::Up => head.move_up(),
            Direction::Down => head.move_down(),
        };

        self.tail.splice(0..0, vec![head].iter().copied());
        self.tail.pop().expect("could not remove last element");
    }
}
