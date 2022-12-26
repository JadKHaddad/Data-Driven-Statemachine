use crate::state::State;
use parking_lot::RwLock;
use std::{error::Error as StdError, sync::Arc};

pub struct StateOption {
    pub name: String,
    pub state: Option<Arc<RwLock<State>>>,
    pub submit: bool,
    pub reset: bool,
}

impl StateOption {
    pub fn new(
        name: String,
        state: Option<Arc<RwLock<State>>>,
        submit: bool,
        reset: bool,
    ) -> StateOption {
        StateOption {
            name,
            state,
            submit,
            reset,
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

    // if reset is defined, reset the index of the state
    pub fn get_state(&mut self) -> Result<Option<Arc<RwLock<State>>>, Box<dyn StdError>> {
        if let Some(state) = &self.state {
            let mut nxt = state.write().into_state_sandwich()?;
            if nxt.is_some() {
                let s = nxt.take().unwrap();
                if self.reset {
                    s.write().reset_index();
                }
                return Ok(Some(s));
            } else {
                if self.reset {
                    self.state.as_ref().unwrap().write().reset_index();
                }
                return Ok(self.state.clone());
            }
        }
        Ok(None)
    }

    pub fn get_submit(&self) -> bool {
        self.submit
    }
}
