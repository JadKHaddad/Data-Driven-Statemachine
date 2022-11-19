use crate::OptionRcRefCellDynStateLike;

pub struct Status {
    pub state_changed: bool,
    pub state: OptionRcRefCellDynStateLike,
    pub submit: bool,
    pub input_recognized: bool,
}

impl Status {
    pub fn new() -> Status {
        Status {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: false,
        }
    }
}

impl std::fmt::Display for Status {
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
