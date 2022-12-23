use crate::state::State;
use parking_lot::RwLock;
use std::sync::Arc;

pub trait StatusLike {
    fn set_state_changed(&mut self, state_changed: bool);
    fn set_state(&mut self, state: Option<Arc<RwLock<State>>>);
    fn set_submit(&mut self, submit: bool);
}

pub struct InputStatus {
    pub state_changed: bool,
    pub state: Option<Arc<RwLock<State>>>,
    pub submit: bool,
    pub input_recognized: bool,
}

impl StatusLike for InputStatus {
    fn set_state_changed(&mut self, state_changed: bool) {
        self.state_changed = state_changed;
    }
    fn set_state(&mut self, state: Option<Arc<RwLock<State>>>) {
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
                name = state.read().get_name();
            }
        }
        write!(
            f,
            "state_changed: {}\nsubmit: {}\nstate name: {}\ninput_recognized: {}",
            self.state_changed, self.submit, name, self.input_recognized
        )
    }
}

#[derive(Default, Debug)]
pub struct Output {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub help: String,
}

impl Output {
    pub fn new(title: String, description: String, options: Vec<String>, help: String) -> Self {
        Self {
            title,
            description,
            options,
            help,
        }
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "title: {}\ndescription: {}\noptions: {:?}\nhelp: {}",
            self.title, self.description, self.options, self.help
        )
    }
}

pub struct OutputStatus {
    pub state_changed: bool,
    pub state: Option<Arc<RwLock<State>>>,
    pub submit: bool,
    pub output: Option<Output>,
}

impl StatusLike for OutputStatus {
    fn set_state_changed(&mut self, state_changed: bool) {
        self.state_changed = state_changed;
    }
    fn set_state(&mut self, state: Option<Arc<RwLock<State>>>) {
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
                name = state.read().get_name();
            }
        }
        let mut output = String::from("None");
        if let Some(out) = &self.output {
            output = format!("{}", out);
        }
        write!(
            f,
            "state_changed: {}\nsubmit: {}\nstate name: {}\noutput: {}",
            self.state_changed, self.submit, name, output
        )
    }
}
