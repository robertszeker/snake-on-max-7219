#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
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
