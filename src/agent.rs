use std::collections::HashMap;

use crate::action::Action;
use crate::state::State;

pub struct Agent {
    pub id: i32,
    pub policy: HashMap<State, Action>,
    pub score: f32,
}
