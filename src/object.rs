use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Object {
    Empty,
    Can,
    Wall,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '_',
            Self::Can => 'O',
            Self::Wall => '#',
        };
        write!(f, "{}", c)
    }
}
