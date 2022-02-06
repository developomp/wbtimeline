use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub is_something: bool,
}

impl State {
    pub fn toggle_something(&mut self) {
        self.is_something = !self.is_something;
    }
}
