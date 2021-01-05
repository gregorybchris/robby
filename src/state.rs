use crate::object::Object;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct State {
    pub up: Object,
    pub down: Object,
    pub left: Object,
    pub right: Object,
    pub center: Object,
}
