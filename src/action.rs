use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Tag(usize),
    Move(Position)
}