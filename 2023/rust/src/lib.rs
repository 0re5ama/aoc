use enum_iterator::Sequence;

#[derive(Sequence, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
