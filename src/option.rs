use crate::state::State;
use parking_lot::RwLock;
use std::{error::Error as StdError, sync::Arc};

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

    pub fn get_state(&mut self) -> Result<Option<Arc<RwLock<State>>>, Box<dyn StdError>> {
        self.state.into_state_sandwich()
    }

    pub fn get_submit(&self) -> bool {
        self.submit
    }
}
