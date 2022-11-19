use crate::{
    collection::Collection, context_like::StateContext, status::Status,
    OptionRcRefCellDynStateLike, OptionVecBoxDynOptionLike,
};
use std::rc::Rc;

pub trait StateLike {
    fn get_name(&self) -> String;
    fn get_parent(&self) -> OptionRcRefCellDynStateLike;

    fn input(&mut self, input: String) -> Status;
    fn output(&self) -> String;

    fn back(&mut self) -> Status;
    fn collect_contexts(&self) -> Vec<Collection>;
}

pub struct OptionsState {
    pub name: String,
    pub parent: OptionRcRefCellDynStateLike,
    pub options: OptionVecBoxDynOptionLike,
}

impl OptionsState {
    pub fn new(
        name: String,
        parent: OptionRcRefCellDynStateLike,
        options: OptionVecBoxDynOptionLike,
    ) -> OptionsState {
        OptionsState {
            name,
            parent,
            options,
        }
    }
}

pub struct ContextState {
    pub name: String,
    pub index: u32,
    pub parent: OptionRcRefCellDynStateLike,
    pub next: OptionRcRefCellDynStateLike,
    pub contexts: Vec<StateContext>,
}

impl ContextState {
    pub fn new(
        name: String,
        parent: OptionRcRefCellDynStateLike,
        next: OptionRcRefCellDynStateLike,
        contexts: Vec<StateContext>,
    ) -> ContextState {
        ContextState {
            name,
            index: 0,
            parent,
            next,
            contexts,
        }
    }
}

impl StateLike for ContextState {
    fn get_name(&self) -> String {
        self.name.clone()
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
            if self.next.is_none() {
                status.submit = true;
            }
            status.state = self.next.clone();
        }

        return status;
    }

    fn output(&self) -> String {
        let mut output = format!("[{}]\n", self.name);
        if let Some(context) = self.contexts.get(self.index as usize) {
            output = format!("{}{}\n", output, context.name);
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

        if self.index > 0 {
            self.index -= 1;
        }

        if self.index == 0 {
            if self.parent.is_some() {
                status.state_changed = true;
                status.state = self.parent.clone();
            }
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

        if let Some(options) = self.options.as_mut() {
            if let Ok(input_as_u32) = input.parse::<u32>() {
                if input_as_u32 > 0 {
                    if let Some(option) = options.get_mut(input_as_u32 as usize - 1) {
                        status.state_changed = true;
                        status.state = option.get_state();
                        status.submit = option.get_submit();
                        status.input_recognized = true;
                        return status;
                    }
                }
            }
            for option in options.iter_mut() {
                if option.input(&input) {
                    status.state_changed = true;
                    status.state = option.get_state();
                    status.submit = option.get_submit();
                    status.input_recognized = true;
                    break;
                }
            }
        }

        return status;
    }

    fn output(&self) -> String {
        let mut output = format!("[{}]\n", self.name);
        if let Some(options) = &self.options {
            for option in options {
                output = format!("{}{}\n", output, option.get_name());
            }
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
