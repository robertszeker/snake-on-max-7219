use super::point::Point;
use super::Direction;
use std::fmt;

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    pub tail: Vec<Point>,
}

#[derive(Debug)]
pub struct GameOver {
    score: usize,
}

impl fmt::Display for GameOver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "game over with score: {}", self.score)
    }
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

    pub fn walk(&mut self) -> Result<(), GameOver> {
        let mut head = Point { ..self.tail[0] };
        match self.direction {
            Direction::Left => head.move_left(),
            Direction::Right => head.move_right(),
            Direction::Up => head.move_up(),
            Direction::Down => head.move_down(),
        };

        self.tail.pop().expect("could not remove last element");

        if self.tail.contains(&head) {
            return Err(GameOver{score: self.tail.len()});
        }

        self.tail.splice(0..0, vec![head].iter().copied());

        Ok(())
    }
}
