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

fn init_display() -> Max7219 {

    let data_pin = 10; 
    let cs_pin = 8; 
    let clk_pin = 11;

    println!("data={}, cs={}, clk={}", data_pin, cs_pin, clk_pin);

    let mut display = setup("/dev/gpiochip0", NUMBER_DISPLAYS, data_pin, cs_pin, clk_pin);
    println!("prepare_display");
    prepare_display(&mut display, NUMBER_DISPLAYS, 0x0F);
    return display;   
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
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
    let args: Vec<String> = std::env::args().collect();

    let mut point = Point {
        x: args[1].parse::<u8>().unwrap(),
        y: args[2].parse::<u8>().unwrap(),
    };

    let stdout = stdout();
    let mut _stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    let mut direction = "left";

    loop {

        let b = stdin.next();

        if let Some(Ok(27)) = b {
            let b = stdin.next();
            if let Some(Ok(91)) = b {
                let b = stdin.next();

                if let Some(Ok(65)) = b {
                    direction = "up";
                } else if let Some(Ok(66)) = b {
                    direction = "down";
                } else if let Some(Ok(67)) = b {
                    direction = "right";
                } else if let Some(Ok(68)) = b {
                    direction = "left";
                }
            }
        }

        if let Some(Ok(b'q')) = b {
            break;
        }

        if direction == "left" {
            point.move_left();
        } else if direction == "up" {
            point.move_up();
        } else if direction == "down" {
            point.move_down();
        } else if direction == "right" {
            point.move_right();
        }

        for i in 0..NUMBER_DISPLAYS {
            display.write_raw(i, &point.get_byte_rows_for_display(i));
        }
    
        thread::sleep(Duration::from_millis(100));
    }
}
