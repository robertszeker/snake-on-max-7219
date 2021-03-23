mod mappings;

use mappings::*;

pub fn encode(s: String) -> Vec<[u8; 8]> {
    s.chars().into_iter().map(|c| map(c)).collect()
}

fn map(c: char) -> [u8; crate::DISPLAY_SIZE] {
    match c {
        '0' => ZERO,
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        _ => SPACE,
    }
}
