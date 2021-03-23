use crate::display::{empty_bytes, DisplayTrait};
use crate::encoding;

mod string_encoder {}

pub struct Score {
    pub score: usize,
}

impl DisplayTrait for Score {
    fn get_bytes(&self, display_index: usize) -> [u8; 8] {
        let string = self.score.to_string();
        let bytes: Vec<[u8; 8]> = encoding::encode(string);
        *bytes.get(display_index).unwrap_or(&empty_bytes())
    }
}
