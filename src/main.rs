use std::io::{Read, stdout};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use crate::types::point::Point;
use crate::types::snake::Snake;
use crate::types::Direction;

const NUMBER_DISPLAYS: usize = 4;
const DISPLAY_SIZE: u8 = 8;
const DATA_PIN: u32 = 10;
const CS_PIN: u32 = 8;
const CLK_PIN: u32 = 11;

mod types {

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
            pub direction: Direction,
            pub tail: Vec<Point>,
        }

        impl Snake {
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
}

mod display {
    use crate::types::snake::Snake;
    use crate::types::point::Point;
    use max_7219_led_matrix_util::setup::{setup, Max7219};
    use max_7219_led_matrix_util::prepare_display;

    pub fn get_byte_rows_for_snake(snake: &Snake, display_index: usize) -> [u8; 8] {
        let byte_row: u8 = 0b00000000;
        let mut byte_rows = [byte_row; 8];

        for point in snake.tail.iter() {
            for byte_row_index in 0..8 {
                byte_rows[byte_row_index] = get_byte_rows_for_point(&point, display_index)[byte_row_index] | byte_rows[byte_row_index];
            }
        }
        byte_rows
    }

    pub fn get_byte_rows_for_point(point: &Point, display_index: usize) -> [u8; 8] {
        let byte_row = 0b00000000;
        let empty_byte_rows = [byte_row; 8];

        let actual_display_index = usize::from(point.x) / usize::from(crate::DISPLAY_SIZE);
        if actual_display_index != display_index {
            return empty_byte_rows;
        }

        let mut byte_rows = empty_byte_rows;
        let column_number = point.x % crate::DISPLAY_SIZE;
        let byte_row = 0b10000000 >> column_number;
        let byte_rows_index = 7 - usize::from(point.y);
        byte_rows[byte_rows_index] = byte_row;
        return byte_rows;
    }

    pub struct Display {
        max7219: Max7219
    }

    impl Display {
        pub fn write(&mut self, snake: &Snake) -> () {
            for i in 0..crate::NUMBER_DISPLAYS {
                self.max7219.write_raw(
                    i,
                    &get_byte_rows_for_snake(&snake, i),
                ).expect("couldn't write to display");
            }
        }
    }

    pub fn init_display() -> crate::display::Display {
        let mut max7219 = setup("/dev/gpiochip0", crate::NUMBER_DISPLAYS, crate::DATA_PIN, crate:: CS_PIN, crate::CLK_PIN);
        prepare_display(&mut max7219, crate::NUMBER_DISPLAYS, 0x0F);
        return Display { max7219 };
    }
}

fn main() {
    let mut display = crate::display::init_display();

    let point1 = Point { x: 8, y: 4 };
    let point2 = Point { x: 9, y: 4 };
    let point3 = Point { x: 10, y: 4 };
    let mut snake = Snake {
        direction: Direction::Left,
        tail: vec![
            point1,
            point2,
            point3,
        ],
    };

    let stdout = stdout();
    let mut _stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        match stdin.next() {
            Some(Ok(27)) => {
                match stdin.next() {
                    Some(Ok(91)) => {
                        match stdin.next() {
                            Some(Ok(65)) => snake.change_direction(Direction::Up),
                            Some(Ok(66)) => snake.change_direction(Direction::Down),
                            Some(Ok(67)) => snake.change_direction(Direction::Right),
                            Some(Ok(68)) => snake.change_direction(Direction::Left),
                            _ => (),
                        }
                    }
                    _ => (),
                }
            },
            Some(Ok(b'q')) => break,
            _ => (),
        };

        snake.walk();
        display.write(&snake);
    
        thread::sleep(Duration::from_millis(100));
    }
}
