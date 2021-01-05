use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Object {
    Empty,
    Goal,
    Wall,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '_',
            Self::Goal => 'O',
            Self::Wall => '#',
        };
        write!(f, "{}", c)
    }
}
