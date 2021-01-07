use crate::object::Object;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct State {
    pub up: Object,
    pub down: Object,
    pub left: Object,
    pub right: Object,
    pub center: Object,
}
