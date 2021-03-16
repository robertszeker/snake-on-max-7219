mod display;
mod input;
mod types;

use rand::rngs::ThreadRng;
use rand::Rng;
use std::io::stdout;
use std::{thread, time::Duration};
use termion::raw::IntoRawMode;
use types::{point::Point, snake::Snake, snake::Status};

const NUMBER_DISPLAYS: usize = 4;
const DISPLAY_SIZE: usize = 8;
const DATA_PIN: u32 = 10;
const CS_PIN: u32 = 8;
const CLK_PIN: u32 = 11;

pub fn generate_mouse(randomizer: &mut ThreadRng, exclude: &Vec<Point>) -> Point {
    let new = Point {
        x: randomizer.gen_range(0..(DISPLAY_SIZE * NUMBER_DISPLAYS)),
        y: randomizer.gen_range(0..DISPLAY_SIZE),
    };

    if exclude.contains(&new) {
        return generate_mouse(randomizer, exclude);
    }

    new
}

fn main() {
    let mut display = display::init();
    let mut snake = Snake::init(vec![
        Point { x: 8, y: 4 },
        Point { x: 9, y: 4 },
        Point { x: 10, y: 4 },
        Point { x: 10, y: 4 },
        Point { x: 10, y: 4 },
        Point { x: 10, y: 4 },
        Point { x: 10, y: 4 },
    ]);

    let stdout = stdout();
    let _stdout = stdout.lock().into_raw_mode().unwrap();
    let mut input = input::Input::init();

    let mut mouse = generate_mouse(&mut rand::thread_rng(), &snake.tail);

    loop {
        match input.next() {
            Some(input::Command::ChangeDirection { direction }) => {
                snake.change_direction(direction)
            }
            Some(input::Command::Break) => break,
            _ => (),
        }

        match snake.walk(&mouse) {
            Status::Walking => {}
            Status::Eating => {
                mouse = generate_mouse(&mut rand::thread_rng(), &snake.tail);
            }
            Status::GameOver(score) => {
                println!("game over with score: {}", score);
                break;
            }
        }
        display.write(vec![&snake, &mouse]);

        thread::sleep(Duration::from_millis(100));
    }
}
