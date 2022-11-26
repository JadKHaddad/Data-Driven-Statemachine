use crate::OptionRcRefCellDynStateLike;

pub trait StatusLike {
    fn set_state_changed(&mut self, state_changed: bool);
    fn set_state(&mut self, state: OptionRcRefCellDynStateLike);
    fn set_submit(&mut self, submit: bool);
}

pub struct InputStatus {
    pub state_changed: bool,
    pub state: OptionRcRefCellDynStateLike,
    pub submit: bool,
    pub input_recognized: bool,
}

impl StatusLike for InputStatus {
    fn set_state_changed(&mut self, state_changed: bool) {
        self.state_changed = state_changed;
    }
    fn set_state(&mut self, state: OptionRcRefCellDynStateLike) {
        self.state = state;
    }
    fn set_submit(&mut self, submit: bool) {
        self.submit = submit;
    }
}

impl std::fmt::Display for InputStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = String::new();
        if self.state_changed {
            if let Some(state) = &self.state {
                name = state.borrow().get_name();
            }
        }
        write!(
            f,
            "state_changed: {}\nsubmit: {}\nstate name: {}\ninput_recognized: {}",
            self.state_changed, self.submit, name, self.input_recognized
        )
    }
}

pub struct OutputStatus {
    pub state_changed: bool,
    pub state: OptionRcRefCellDynStateLike,
    pub submit: bool,
    pub output: String,
}

impl StatusLike for OutputStatus {
    fn set_state_changed(&mut self, state_changed: bool) {
        self.state_changed = state_changed;
    }
    fn set_state(&mut self, state: OptionRcRefCellDynStateLike) {
        self.state = state;
    }
    fn set_submit(&mut self, submit: bool) {
        self.submit = submit;
    }
}

impl std::fmt::Display for OutputStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = String::new();
        if self.state_changed {
            if let Some(state) = &self.state {
                name = state.borrow().get_name();
            }
        }
        write!(
            f,
            "state_changed: {}\nsubmit: {}\nstate name: {}\noutput: {}",
            self.state_changed, self.submit, name, self.output
        )
    }
}
