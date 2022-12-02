use crate::{BoxDynIntoStateLike, OptionRcRefCellDynStateLike};
use std::error::Error as StdError;

pub trait OptionLike {
    fn input(&self, input: &String) -> bool;
    fn get_name(&self) -> String;
    fn get_state(&mut self) -> Result<OptionRcRefCellDynStateLike, Box<dyn StdError>>;
    fn get_submit(&self) -> bool;
}

pub struct StateOption {
    pub name: String,
    pub state: BoxDynIntoStateLike,
    pub submit: bool,
}

impl StateOption {
    pub fn new(name: String, state: BoxDynIntoStateLike, submit: bool) -> StateOption {
        StateOption {
            name,
            state,
            submit,
        }
    }
}

impl OptionLike for StateOption {
    fn input(&self, input: &String) -> bool {
        if &self.name == input {
            return true;
        }
        false
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_state(&mut self) -> Result<OptionRcRefCellDynStateLike, Box<dyn StdError>> {
        self.state.into_state_like()
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}
