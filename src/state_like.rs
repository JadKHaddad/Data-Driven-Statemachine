use crate::{
    collection::Collection, context_like::StateContext, option_like::OptionLike, status::Status,
    OptionBoxDynIntoStateLike, OptionRcRefCellDynStateLike, RcRefCellContextState,
    RcRefCellDynStateLike, RcRefCellOptionsState, VecBoxDynContextLike, VecBoxDynOptionLike,
};
use std::rc::Rc;

pub trait StateLike {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_parent(&self) -> OptionRcRefCellDynStateLike;
    fn input(&mut self, input: String) -> Status;
    fn output(&mut self) -> String;
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
    pub fn new(
        closure_state: impl Fn() -> RcRefCellDynStateLike + 'static,
        lazy: bool,
    ) -> StateHolder {
        let mut state_holder = StateHolder {
            closure_state: Box::new(closure_state),
            state: None,
        };
        if !lazy {
            state_holder.into_state_like();
        }
        state_holder
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
    pub next: OptionBoxDynIntoStateLike,
    pub contexts: VecBoxDynContextLike,
    pub submit: bool,
}

impl ContextState {
    pub fn new(
        name: String,
        description: String,
        parent: OptionRcRefCellDynStateLike,
        next: OptionBoxDynIntoStateLike,
        contexts: VecBoxDynContextLike,
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
        dbg!(self.index);
        //submit will be true if all contexts are filled and the next state is not set
        //if the next state is set, then the submit will be the state's submit value
        let mut status = Status {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: true,
        };



        if let Some(context) = self.contexts.get_mut(self.index as usize) {
            dbg!(context.get_name());
            context.input(input);
            
            if let Some(context) = self.contexts.get_mut(self.index as usize + 1) {
                let next_state = context.output();
                if next_state.is_some() {
                    dbg!("next state is some");
                    status.state = next_state;
                    status.state_changed = true;
                }
            }
        }

        if self.index < self.contexts.len() as u32 {
            dbg!("increasing index");
            self.index += 1;
        }


        if status.state.is_some() {
            dbg!("returning options");
            return status;
        }

        if self.index >= self.contexts.len() as u32 {
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

    fn output(&mut self) -> String {
        // if let Some(context) = self.contexts.get_mut(self.index as usize) {
        //     if let Some(state) = context.output() {
        //         return state.borrow_mut().output();
        //     }
        // }
        let mut output = format!("[{}]\n", self.name);
        //add description if index is 0
        if self.index == 0 {
            output.push_str(&format!("{}\n", self.description));
        }
        if let Some(context) = self.contexts.get(self.index as usize) {
            output.push_str(&format!("{}\n", context.get_name()));
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
    //VecBoxDynContextLike
    fn collect_contexts(&self) -> Vec<Collection> {
        let context_collections = self
            .contexts
            .iter()
            .map(|context| context.collect())
            .collect();

        let collection = Collection {
            name: self.name.clone(),
            context_collections,
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

    fn output(&mut self) -> String {
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
            context_collections: Vec::new(),
        };
        vec![collection]
    }
}
