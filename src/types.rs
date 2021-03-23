pub mod point;
pub mod score;
pub mod snake;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
