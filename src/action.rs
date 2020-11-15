use iced::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Tag(usize),
    Move(Point)
}