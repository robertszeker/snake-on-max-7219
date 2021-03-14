use super::types::{point::Point, snake::Snake};
use max_7219_led_matrix_util::{
    prepare_display,
    setup::{setup, Max7219},
};

pub fn get_byte_rows_for_snake(snake: &Snake, display_index: usize) -> [u8; 8] {
    let byte_row: u8 = 0b00000000;
    let mut byte_rows = [byte_row; 8];

    for point in snake.tail.iter() {
        for byte_row_index in 0..8 {
            byte_rows[byte_row_index] = get_byte_rows_for_point(&point, display_index)
                [byte_row_index]
                | byte_rows[byte_row_index];
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
    max7219: Max7219,
}

impl Display {
    pub fn write(&mut self, snake: &Snake) -> () {
        for i in 0..crate::NUMBER_DISPLAYS {
            self.max7219
                .write_raw(i, &get_byte_rows_for_snake(&snake, i))
                .expect("couldn't write to display");
        }
    }
}

pub fn init() -> Display {
    let mut max7219 = setup(
        "/dev/gpiochip0",
        crate::NUMBER_DISPLAYS,
        crate::DATA_PIN,
        crate::CS_PIN,
        crate::CLK_PIN,
    );
    prepare_display(&mut max7219, crate::NUMBER_DISPLAYS, 0x0F);
    return Display { max7219 };
}
