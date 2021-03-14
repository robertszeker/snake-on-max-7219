mod display;
mod types;

use std::io::Read;
use std::{thread, time::Duration};
use termion::async_stdin;
use types::{point::Point, snake::Snake, Direction};

const NUMBER_DISPLAYS: usize = 4;
const DISPLAY_SIZE: u8 = 8;
const DATA_PIN: u32 = 10;
const CS_PIN: u32 = 8;
const CLK_PIN: u32 = 11;

fn main() {
    let mut display = display::init();
    let mut snake = Snake::init(vec![
        Point { x: 8, y: 4 },
        Point { x: 9, y: 4 },
        Point { x: 10, y: 4 },
    ]);

    let mut stdin = async_stdin().bytes();

    loop {
        match stdin.next() {
            Some(Ok(27)) => match stdin.next() {
                Some(Ok(91)) => match stdin.next() {
                    Some(Ok(65)) => snake.change_direction(Direction::Up),
                    Some(Ok(66)) => snake.change_direction(Direction::Down),
                    Some(Ok(67)) => snake.change_direction(Direction::Right),
                    Some(Ok(68)) => snake.change_direction(Direction::Left),
                    _ => (),
                },
                _ => (),
            },
            Some(Ok(b'q')) => break,
            _ => (),
        };

        snake.walk();
        display.write(&snake);

        thread::sleep(Duration::from_millis(100));
    }
}
