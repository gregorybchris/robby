use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveRandom,
    PickUp,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::MoveUp => 'U',
            Self::MoveDown => 'D',
            Self::MoveLeft => 'L',
            Self::MoveRight => 'R',
            Self::MoveRandom => '?',
            Self::PickUp => 'P',
        };
        write!(f, "{}", c)
    }
}
