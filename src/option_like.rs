use crate::{State, OptionArcRwLockState};
use std::error::Error as StdError;

#[derive(Clone)]
pub struct StateOption {
    pub name: String,
    pub state: State,
    pub submit: bool,
}

impl StateOption {
    pub fn new(name: String, state: State, submit: bool) -> StateOption {
        StateOption {
            name,
            state,
            submit,
        }
    }
    pub fn input(&self, input: &String) -> bool {
        if &self.name == input {
            return true;
        }
        false
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_state(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        self.state.into_state_like()
    }

    pub fn get_submit(&self) -> bool {
        self.submit
    }
}
