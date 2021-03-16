use max_7219_led_matrix_util::{
    prepare_display,
    setup::{setup, Max7219},
};
use crate::DISPLAY_SIZE;


pub trait DisplayTrait {
    fn get_bytes(&self, display_index: usize) -> [u8; 8];
}

pub struct Display {
    max7219: Max7219,
}

pub fn empty_bytes() -> [u8; 8] {
    let byte_row = 0b00000000;
    [byte_row; DISPLAY_SIZE]
}

impl Display {
    pub fn write(&mut self, objects: Vec<&dyn DisplayTrait>) -> () {
        for display_index in 0..crate::NUMBER_DISPLAYS {
            let mut bytes = empty_bytes();
            for object in objects.iter() {
                for (bytes_index, object_bytes) in object.get_bytes(display_index).iter().enumerate() {
                    bytes[bytes_index] |= object_bytes;
                }
            }
            self.max7219
                .write_raw(display_index, &bytes)
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
