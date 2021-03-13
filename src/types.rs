#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub mod snake {
    use super::Direction;
    use super::point::Point;

    #[derive(Debug)]
    pub struct Snake {
        direction: Direction,
        pub tail: Vec<Point>,
    }

    impl Snake {
        pub fn init(tail: Vec<Point>) -> Snake {
            Snake {tail, direction: Direction::Left}
        }

        pub fn change_direction(&mut self, direction: Direction) -> () {
            self.direction = direction;
        }

        pub fn walk(&mut self) -> () {
            let mut head = Point{..self.tail[0]};
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
}

pub mod point {
    use std::convert::TryFrom;

    #[derive(Debug, Copy, Clone)]
    pub struct Point {
        pub x: u8,
        pub y: u8,
    }

    impl Point {
        pub fn move_left(&mut self) -> () {
            if self.x == 0 {
                self.x = u8::try_from(crate::NUMBER_DISPLAYS).unwrap() * crate::DISPLAY_SIZE;
            }

            self.x = self.x - 1;
        }

        pub fn move_down(&mut self) -> () {
            if self.y == 0 {
                self.y = crate::DISPLAY_SIZE;
            }

            self.y = self.y - 1;
        }

        pub fn move_up(&mut self) -> () {
            if self.y == crate::DISPLAY_SIZE - 1 {
                self.y = 0;
            } else {
                self.y = self.y + 1;
            }
        }

        pub fn move_right(&mut self) -> () {
            if self.x == (u8::try_from(crate::NUMBER_DISPLAYS).unwrap() * crate::DISPLAY_SIZE - 1) {
                self.x = 0;
            } else {
                self.x = self.x + 1;
            }
        }
    }
}
