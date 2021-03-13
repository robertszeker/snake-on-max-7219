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