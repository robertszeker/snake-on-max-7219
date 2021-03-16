use crate::display::DisplayTrait;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl DisplayTrait for Point {
    fn get_bytes(&self, display_index: usize) -> [u8; 8] {
        let byte_row = 0b00000000;
        let empty_byte_rows = [byte_row; 8];

        let actual_display_index = self.x / crate::DISPLAY_SIZE;
        if actual_display_index != display_index {
            return empty_byte_rows;
        }

        let mut byte_rows = empty_byte_rows;
        let column_number = self.x % crate::DISPLAY_SIZE;
        let byte_row = 0b10000000 >> column_number;
        let byte_rows_index = 7 - self.y;
        byte_rows[byte_rows_index] = byte_row;

        byte_rows
    }
}

impl Point {
    pub fn move_left(&mut self) -> () {
        if self.x == 0 {
            self.x = crate::NUMBER_DISPLAYS * crate::DISPLAY_SIZE;
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
        if self.x == crate::NUMBER_DISPLAYS * crate::DISPLAY_SIZE - 1 {
            self.x = 0;
        } else {
            self.x = self.x + 1;
        }
    }
}
