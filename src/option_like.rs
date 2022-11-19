use crate::{state_like::IntoStateLike, OptionRcRefCellDynStateLike};

//maybe obsolet
pub trait OptionLike {
    fn input(&self, input: &String) -> bool;
    fn get_name(&self) -> String;
    fn get_state(&mut self) -> OptionRcRefCellDynStateLike;
    fn get_submit(&self) -> bool;
}

pub struct StateOption {
    pub name: String,
    pub state: Box<dyn IntoStateLike>,
    pub submit: bool,
}

impl StateOption {
    pub fn new(name: String, state: Box<dyn IntoStateLike>, submit: bool) -> StateOption {
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

    fn get_state(&mut self) -> OptionRcRefCellDynStateLike {
        self.state.into_state_like()
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}

/*
pub struct StateClosureOption {
    pub name: String,
    pub closure_state: Box<dyn Fn() -> RcRefCellDynStateLike>, //parent should be passed in
    pub submit: bool,
    pub state: OptionRcRefCellDynStateLike,
}

impl StateClosureOption {
    pub fn new(
        name: String,
        closure_state: impl Fn() -> RcRefCellDynStateLike + 'static,
        submit: bool,
    ) -> StateClosureOption {
        StateClosureOption {
            name,
            closure_state: Box::new(closure_state),
            submit,
            state: None,
        }
    }
}

impl OptionLike for StateClosureOption {
    fn input(&self, input: &String) -> bool {
        if &self.name == input {
            return true;
        }
        false
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_state(&mut self) -> RcRefCellDynStateLike {
        if let Some(state) = &self.state {
            return state.clone();
        }
        let state = (self.closure_state)();
        self.state = Some(state.clone());
        state
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}
*/
