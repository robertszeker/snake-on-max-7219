use max_7219_led_matrix_util::setup::{setup, Max7219};
use max_7219_led_matrix_util::prepare_display;
use std::convert::TryFrom;
use std::io::{Read, stdout};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::async_stdin;

const NUMBER_DISPLAYS: usize = 4;
const DISPLAY_SIZE: u8 = 8;
const DATA_PIN: u32 = 10;
const CS_PIN: u32 = 8;
const CLK_PIN: u32 = 11;

fn init_display() -> Max7219 {
    let mut display = setup("/dev/gpiochip0", NUMBER_DISPLAYS, DATA_PIN, CS_PIN, CLK_PIN);
    prepare_display(&mut display, NUMBER_DISPLAYS, 0x0F);
    return display;   
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u8,
    y: u8,
}

struct Snake {
    direction: Direction,
    tail: Vec<Point>,
}

impl Snake {
    fn change_direction(&mut self, direction: Direction) -> () {
        self.direction = direction;
    }

    fn get_byte_rows_for_display(&self, display_index: usize) -> [u8; 8] {
        let byte_row: u8 = 0b00000000;
        let mut byte_rows = [byte_row; 8];

        for point in self.tail {
            for byte_row_index in 0..8 {
                byte_rows[byte_row_index] = point.get_byte_rows_for_display(display_index)[byte_row_index] & byte_rows[byte_row_index];
            }
        }

        byte_rows
    }

    fn walk(&mut self) -> () {
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

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Point {
    fn get_byte_rows_for_display(&self, display_index: usize) -> [u8; 8] {
        let byte_row = 0b00000000;
        let empty_byte_rows = [byte_row; 8];

        let actual_display_index = usize::from(self.x) / usize::from(DISPLAY_SIZE);
        if actual_display_index != display_index {
            return empty_byte_rows;
        }

        let mut byte_rows = empty_byte_rows;
        let column_number = self.x % DISPLAY_SIZE;
        let byte_row = 0b10000000 >> column_number;
        let byte_rows_index = 7 - usize::from(self.y);
        byte_rows[byte_rows_index] = byte_row;
        return byte_rows;
    }

    fn move_left(&mut self) -> () {
        if self.x == 0 {
            self.x = u8::try_from(NUMBER_DISPLAYS).unwrap() * DISPLAY_SIZE;
        }

        self.x = self.x - 1;
    }

    fn move_down(&mut self) -> () {
        if self.y == 0 {
            self.y = DISPLAY_SIZE;
        }

        self.y = self.y - 1;
    }

    fn move_up(&mut self) -> () {
        if self.y == DISPLAY_SIZE - 1 {
            self.y = 0;
        } else {
            self.y = self.y + 1;
        }
    }

    fn move_right(&mut self) -> () {
        if self.x == (u8::try_from(NUMBER_DISPLAYS).unwrap() * DISPLAY_SIZE - 1) {
            self.x = 0;
        } else {
            self.x = self.x + 1;
        }
    }
}

fn main() {
    let mut display = init_display();

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

        // snake.walk();

        for i in 0..NUMBER_DISPLAYS {
            // display.write_raw(i, &point.get_byte_rows_for_display(i)).expect("couldn't write to display");
            display.write_raw(i, &snake.get_byte_rows_for_display(i)).expect("couldn't write to display");
        }
    
        thread::sleep(Duration::from_millis(100));
    }
}
