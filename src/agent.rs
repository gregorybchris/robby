use std::collections::BTreeMap;

use crate::action::Action;
use crate::state::State;

pub struct Agent {
    pub id: i32,
    pub policy: BTreeMap<State, Action>,
    pub score: f32,
}
