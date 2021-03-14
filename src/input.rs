use crate::types::Direction;
use std::io::{Bytes, Read};
use termion::{async_stdin, AsyncReader};

impl Input {
    pub fn init() -> Input {
        Input {
            stdin: async_stdin().bytes(),
        }
    }

    pub fn next(&mut self) -> Option<Command> {
        match self.stdin.next() {
            Some(Ok(27)) => match self.stdin.next() {
                Some(Ok(91)) => match self.stdin.next() {
                    Some(Ok(65)) => Some(Command::ChangeDirection {
                        direction: Direction::Up,
                    }),
                    Some(Ok(66)) => Some(Command::ChangeDirection {
                        direction: Direction::Down,
                    }),
                    Some(Ok(67)) => Some(Command::ChangeDirection {
                        direction: Direction::Right,
                    }),
                    Some(Ok(68)) => Some(Command::ChangeDirection {
                        direction: Direction::Left,
                    }),
                    _ => None,
                },
                _ => None,
            },
            Some(Ok(b'q')) => Some(Command::Break),
            _ => None,
        }
    }
}

pub struct Input {
    stdin: Bytes<AsyncReader>,
}

pub enum Command {
    Break,
    ChangeDirection { direction: Direction },
}
