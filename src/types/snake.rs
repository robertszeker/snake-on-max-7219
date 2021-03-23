use super::point::Point;
use super::score::Score;
use super::Direction;
use crate::display::{empty_bytes, DisplayTrait};

pub enum Status {
    Walking,
    Eating,
    GameOver(Score),
}

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    pub tail: Vec<Point>,
    initial_length: usize,
}

const HORIZONTAL_DIRECTION: [Direction; 2] = [Direction::Left, Direction::Right];
const VERTICAL_DIRECTION: [Direction; 2] = [Direction::Up, Direction::Down];

impl DisplayTrait for Snake {
    fn get_bytes(&self, display_index: usize) -> [u8; 8] {
        let mut byte_rows = empty_bytes();
        for point in self.tail.iter() {
            for byte_row_index in 0..8 {
                byte_rows[byte_row_index] |= &point.get_bytes(display_index)[byte_row_index];
            }
        }
        byte_rows
    }
}

impl Snake {
    pub fn init(tail: Vec<Point>) -> Snake {
        Snake {
            initial_length: tail.len(),
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
            return Status::GameOver(Score {
                score: self.tail.len() + 1 - self.initial_length,
            });
        }

        self.tail.splice(0..0, vec![head].iter().copied());

        Status::Walking
    }
}
