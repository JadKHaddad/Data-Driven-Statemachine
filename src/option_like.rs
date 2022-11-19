use crate::OptionRcRefCellDynStateLike;

pub trait OptionLike {
    fn input(&self, input: &String) -> bool;
    fn get_name(&self) -> String;
    fn get_state(&mut self) -> OptionRcRefCellDynStateLike;
    fn get_submit(&self) -> bool;
}

pub struct StateOption {
    pub name: String,
    pub state: OptionRcRefCellDynStateLike,
    pub submit: bool,
}

pub struct StateClosureOption {
    pub name: String,
    pub closure_state: Box<dyn Fn() -> OptionRcRefCellDynStateLike>, //parent should be passed in
    pub submit: bool,
    pub state_created: bool,
    pub state: OptionRcRefCellDynStateLike,
}

impl StateOption {
    pub fn new(name: String, state: OptionRcRefCellDynStateLike, submit: bool) -> StateOption {
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
        self.state.clone()
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}

impl StateClosureOption {
    pub fn new(
        name: String,
        closure_state: impl Fn() -> OptionRcRefCellDynStateLike + 'static,
        submit: bool,
    ) -> StateClosureOption {
        StateClosureOption {
            name,
            closure_state: Box::new(closure_state),
            submit,
            state_created: false,
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

    fn get_state(&mut self) -> OptionRcRefCellDynStateLike {
        if !self.state_created {
            self.state = (self.closure_state)();
            self.state_created = true;
        }

        self.state.clone()
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}
