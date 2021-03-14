mod display;
mod input;
mod types;

use std::{thread, time::Duration};
use types::{point::Point, snake::Snake};

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

    let mut input = input::Input::init();

    loop {
        match input.next() {
            Some(input::Command::ChangeDirection { direction }) => {
                snake.change_direction(direction)
            }
            Some(input::Command::Break) => break,
            _ => (),
        }

        snake.walk();
        display.write(&snake);

        thread::sleep(Duration::from_millis(100));
    }
}
