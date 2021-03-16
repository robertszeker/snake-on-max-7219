use super::point::Point;
use super::Direction;

pub enum Status {
    Walking,
    Eating,
    GameOver(usize),
}

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    pub tail: Vec<Point>,
}

const HORIZONTAL_DIRECTION: [Direction; 2] = [Direction::Left, Direction::Right];
const VERTICAL_DIRECTION: [Direction; 2] = [Direction::Up, Direction::Down];

impl Snake {
    pub fn init(tail: Vec<Point>) -> Snake {
        Snake {
            tail,
            direction: Direction::Left,
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) -> () {
        if HORIZONTAL_DIRECTION.contains(&self.direction)
            && HORIZONTAL_DIRECTION.contains(&new_direction)
        {
            return;
        }

        if VERTICAL_DIRECTION.contains(&self.direction)
            && VERTICAL_DIRECTION.contains(&new_direction)
        {
            return;
        }

        self.direction = new_direction;
    }

    pub fn walk(&mut self, mouse: &Point) -> Status {
        let mut head = Point { ..self.tail[0] };
        match self.direction {
            Direction::Left => head.move_left(),
            Direction::Right => head.move_right(),
            Direction::Up => head.move_up(),
            Direction::Down => head.move_down(),
        };

        if head.eq(mouse) {
            self.tail.splice(0..0, vec![head].iter().copied());
            return Status::Eating;
        } else {
            self.tail.pop().expect("could not remove last element");
        }

        if self.tail.contains(&head) {
            return Status::GameOver(self.tail.len());
        }

        self.tail.splice(0..0, vec![head].iter().copied());

        Status::Walking
    }
}
