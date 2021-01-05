use std::collections::HashMap;

use crate::action::Action;
use crate::state::State;

pub struct Robot {
    pub id: i32,
    pub policy: HashMap<State, Action>,
    pub score: f32,
}
