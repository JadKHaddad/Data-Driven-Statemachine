use crate::{
    collection::Collection, context_like::StateContext, option_like::OptionLike, status::Status,
    OptionRcRefCellDynStateLike, RcRefCellContextState, RcRefCellDynStateLike,
    RcRefCellOptionsState, VecBoxDynOptionLike,
};
use std::rc::Rc;

pub trait StateLike {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_parent(&self) -> OptionRcRefCellDynStateLike;
    fn input(&mut self, input: String) -> Status;
    fn output(&self) -> String;
    fn back(&mut self) -> Status;
    fn collect_contexts(&self) -> Vec<Collection>;
}

pub trait IntoStateLike {
    fn into_state_like(&mut self) -> OptionRcRefCellDynStateLike;
}

impl IntoStateLike for RcRefCellOptionsState {
    fn into_state_like(&mut self) -> OptionRcRefCellDynStateLike {
        Some(self.clone())
    }
}

impl IntoStateLike for RcRefCellContextState {
    fn into_state_like(&mut self) -> OptionRcRefCellDynStateLike {
        Some(self.clone())
    }
}

impl IntoStateLike for RcRefCellDynStateLike {
    fn into_state_like(&mut self) -> OptionRcRefCellDynStateLike {
        Some(self.clone())
    }
}

impl IntoStateLike for StateHolder {
    fn into_state_like(&mut self) -> OptionRcRefCellDynStateLike {
        if let Some(state) = &self.state {
            dbg!("State already exists");
            return Some(state.clone());
        }
        dbg!("State creating");
        let state = (self.closure_state)();
        self.state = Some(state.clone());
        Some(state)
    }
}

pub struct StateHolder {
    pub closure_state: Box<dyn Fn() -> RcRefCellDynStateLike>, //parent should be passed in
    pub state: OptionRcRefCellDynStateLike,
}

impl StateHolder {
    pub fn new(closure_state: impl Fn() -> RcRefCellDynStateLike + 'static) -> StateHolder {
        StateHolder {
            closure_state: Box::new(closure_state),
            state: None,
        }
    }
}
pub struct OptionsState {
    pub name: String,
    pub description: String,
    pub parent: OptionRcRefCellDynStateLike,
    pub options: VecBoxDynOptionLike,
}

impl OptionsState {
    pub fn new(
        name: String,
        description: String,
        parent: OptionRcRefCellDynStateLike,
        options: VecBoxDynOptionLike,
    ) -> OptionsState {
        OptionsState {
            name,
            description,
            parent,
            options,
        }
    }
}

pub struct ContextState {
    pub name: String,
    pub description: String,
    pub index: u32,
    pub parent: OptionRcRefCellDynStateLike,
    pub next: Option<Box<dyn IntoStateLike>>,
    pub contexts: Vec<StateContext>,
    pub submit: bool,
}

impl ContextState {
    pub fn new(
        name: String,
        description: String,
        parent: OptionRcRefCellDynStateLike,
        next: Option<Box<dyn IntoStateLike>>,
        contexts: Vec<StateContext>,
        submit: bool,
    ) -> ContextState {
        ContextState {
            name,
            description,
            index: 0,
            parent,
            next,
            contexts,
            submit,
        }
    }
}

impl StateLike for ContextState {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_parent(&self) -> OptionRcRefCellDynStateLike {
        if let Some(parent) = &self.parent {
            return Some(Rc::clone(&parent));
        }
        None
    }

    fn input(&mut self, input: String) -> Status {
        //submit will be true if all contexts are filled and the next state is not set
        //if the next state is set, then the submit will be the state's submit value
        let mut status = Status {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: true,
        };

        if let Some(mut context) = self.contexts.get_mut(self.index as usize) {
            context.value = input;
        }

        if self.index < self.contexts.len() as u32 {
            self.index += 1;
        }

        if self.index == self.contexts.len() as u32 {
            status.state_changed = true;
            status.submit = self.submit;

            if let Some(next) = &mut self.next {
                dbg!("Next state");
                status.state = next.into_state_like();
            } else {
                dbg!("No next state");
                status.submit = true;
            }
        }

        return status;
    }

    fn output(&self) -> String {
        let mut output = format!("[{}]\n", self.name);
        //add description if index is 0
        if self.index == 0 {
            output.push_str(&format!("{}\n", self.description));
        }
        if let Some(context) = self.contexts.get(self.index as usize) {
            output.push_str(&format!("{}\n", context.name));
        }
        output
    }

    fn back(&mut self) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: true,
        };

        if self.index == 0 {
            if self.parent.is_some() {
                status.state_changed = true;
                status.state = self.parent.clone();
            }
        }

        if self.index > 0 {
            self.index -= 1;
        }

        return status;
    }

    fn collect_contexts(&self) -> Vec<Collection> {
        let collection = Collection {
            name: self.name.clone(),
            contexts: self.contexts.clone(),
        };

        if let Some(parent) = &self.parent {
            let mut parent_collections = parent.borrow().collect_contexts();
            parent_collections.push(collection);
            return parent_collections;
        }

        vec![collection]
    }
}

impl StateLike for OptionsState {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_parent(&self) -> OptionRcRefCellDynStateLike {
        if let Some(parent) = &self.parent {
            return Some(Rc::clone(&parent));
        }
        None
    }

    fn input(&mut self, input: String) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: false,
        };

        fn on_input_recognized(status: &mut Status, option: &mut Box<dyn OptionLike>) {
            status.state_changed = true;
            status.state = option.get_state();
            status.submit = option.get_submit();
            status.input_recognized = true;
        }

        if let Ok(input_as_u32) = input.parse::<u32>() {
            if input_as_u32 > 0 {
                if let Some(option) = self.options.get_mut(input_as_u32 as usize - 1) {
                    on_input_recognized(&mut status, option);
                    return status;
                }
            }
        }
        for option in self.options.iter_mut() {
            if option.input(&input) {
                on_input_recognized(&mut status, option);
                return status;
            }
        }

        return status;
    }

    fn output(&self) -> String {
        let mut output = format!("[{}]\n", self.name);
        output.push_str(&format!("{}\n", self.description));
        for (index, option) in self.options.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", index + 1, option.get_name()));
        }
        output
    }

    fn back(&mut self) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: true,
        };
        if self.parent.is_some() {
            status.state_changed = true;
            status.state = self.parent.clone();
        }
        return status;
    }

    fn collect_contexts(&self) -> Vec<Collection> {
        let collection = Collection {
            name: self.name.clone(),
            contexts: Vec::new(),
        };
        vec![collection]
    }
}
