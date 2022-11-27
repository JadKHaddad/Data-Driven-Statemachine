use crate::{
    collection::{Collection, ContextLikeCollection},
    option_like::OptionLike,
    status::{InputStatus, OutputStatus, StatusLike},
    OptionBoxDynIntoStateLike, OptionRcRefCellDynStateLike, RcRefCellContextState,
    RcRefCellDynStateLike, RcRefCellOptionsState, VecBoxDynContextLike, VecBoxDynOptionLike,
};

pub trait StateLike {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_parent(&self) -> OptionRcRefCellDynStateLike;
    fn get_index(&self) -> usize;
    fn get_options(&mut self) -> Option<&mut VecBoxDynOptionLike>;
    fn get_contexts(&mut self) -> Option<&mut VecBoxDynContextLike>;
    fn decrease_index(&mut self, amount: usize);
    fn input(&mut self, input: String) -> InputStatus;
    fn output(&mut self) -> OutputStatus;
    fn back(&mut self) -> InputStatus;
    fn collect(&mut self) -> Vec<Collection>;
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
    pub index: usize,
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
            index: 0,
            parent,
            options,
        }
    }
}

pub struct ContextState {
    pub name: String,
    pub description: String,
    pub index: usize,
    pub parent: OptionRcRefCellDynStateLike,
    pub next: OptionBoxDynIntoStateLike,
    pub contexts: VecBoxDynContextLike,
    pub submit: bool,
    pub go_back: bool,
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
            go_back: false,
        }
    }

    fn on_highest_index(&mut self, status: &mut impl StatusLike) {
        status.set_state_changed(true);
        status.set_submit(self.submit);

        if let Some(next) = &mut self.next {
            dbg!("Next state");
            status.set_state(next.into_state_like());
        } else {
            dbg!("No next state");
            status.set_submit(true);
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
        self.parent.clone()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_options(&mut self) -> Option<&mut VecBoxDynOptionLike> {
        None
    }

    fn get_contexts(&mut self) -> Option<&mut VecBoxDynContextLike> {
        Some(&mut self.contexts)
    }

    fn input(&mut self, input: String) -> InputStatus {
        //submit will be true if all contexts are filled and the next state is not set
        //if the next state is set, then the submit will be the state's submit value
        let mut status = InputStatus {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: true,
        };

        if let Some(context) = self.contexts.get_mut(self.index) {
            context.input(input);
        }

        if self.index < self.contexts.len() {
            self.index += 1;
        }

        if self.index >= self.contexts.len() {
            self.on_highest_index(&mut status);
        }

        return status;
    }

    fn output(&mut self) -> OutputStatus {
        let mut status = OutputStatus {
            state_changed: false,
            state: None,
            submit: false,
            output: String::new(),
        };

        if self.go_back {
            self.go_back = false;
            status.state_changed = true;
            status.state = self.get_parent();
            return status;
        }

        if self.index >= self.contexts.len() {
            self.on_highest_index(&mut status);
            return status;
        }

        //this means that the current context is an option
        if let Some(context) = self.contexts.get_mut(self.index) {
            let next_state = context.output();
            if next_state.is_some() {
                if self.index < self.contexts.len() {
                    self.index += 1;
                }
                return OutputStatus {
                    state_changed: true,
                    state: next_state,
                    submit: false,
                    output: String::new(),
                };
            }
        }

        //normal output for contexts
        let mut output = format!("[{}]\n", self.name);

        //add description if index is 0
        if self.index == 0 {
            output.push_str(&format!("{}\n", self.description));
        }
        if let Some(context) = self.contexts.get(self.index) {
            output.push_str(&format!("{}\n", context.get_name()));
        }
        OutputStatus {
            state_changed: false,
            state: None,
            submit: false,
            output,
        }
    }

    fn back(&mut self) -> InputStatus {
        let mut status = InputStatus {
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

    fn collect(&mut self) -> Vec<Collection> {
        let collection = Collection {
            state_name: self.get_name(),
            context_collections: self
                .contexts
                .iter_mut()
                .map(|context| context.collect())
                .collect(),
        };

        if let Some(parent) = &self.parent {
            let mut parent_collections = parent.borrow_mut().collect();
            parent_collections.push(collection);
            return parent_collections;
        }

        vec![collection]
    }

    //called from an OptionsState that has been created through a Context
    fn decrease_index(&mut self, amount: usize) {
        if amount > self.index {
            self.go_back = true;
            self.index = 0;
        } else {
            self.index -= amount;
        }
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
        self.parent.clone()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_options(&mut self) -> Option<&mut VecBoxDynOptionLike> {
        Some(&mut self.options)
    }

    fn get_contexts(&mut self) -> Option<&mut VecBoxDynContextLike> {
        None
    }

    fn input(&mut self, input: String) -> InputStatus {
        let mut status = InputStatus {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: false,
        };

        fn on_input_recognized(status: &mut InputStatus, option: &mut Box<dyn OptionLike>) {
            status.state_changed = true;
            status.state = option.get_state();
            status.submit = option.get_submit();
            status.input_recognized = true;
        }

        if let Ok(input_as_u32) = input.parse::<u32>() {
            if input_as_u32 > 0 {
                let index = input_as_u32 as usize - 1;
                if let Some(option) = self.options.get_mut(index) {
                    on_input_recognized(&mut status, option);
                    self.index = index;
                    return status;
                }
            }
        }
        for (index, option) in self.options.iter_mut().enumerate() {
            if option.input(&input) {
                on_input_recognized(&mut status, option);
                self.index = index;
                return status;
            }
        }

        return status;
    }

    fn output(&mut self) -> OutputStatus {
        let mut output = format!("[{}]\n", self.name);
        output.push_str(&format!("{}\n", self.description));
        for (index, option) in self.options.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", index + 1, option.get_name()));
        }
        OutputStatus {
            output,
            state_changed: false,
            submit: false,
            state: None,
        }
    }

    fn back(&mut self) -> InputStatus {
        let mut status = InputStatus {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: true,
        };
        if let Some(parent) = &self.parent {
            status.state_changed = true;
            status.state = self.parent.clone();
            parent.borrow_mut().decrease_index(2);
        }

        return status;
    }

    fn collect(&mut self) -> Vec<Collection> {
        if let Some(option) = self.options.get(self.index) {
            let context_like_collection =
                ContextLikeCollection::new(self.name.clone(), option.get_name());

            let mut collection = Collection {
                state_name: "None".to_string(),
                context_collections: vec![context_like_collection],
            };

            if let Some(parent) = &self.parent {
                let mut parent_mute = parent.borrow_mut();
                collection.state_name = parent_mute.get_name();
                let mut parent_collections = parent_mute.collect();
                parent_collections.push(collection);
                return parent_collections;
            }

            return vec![collection];
        }
        //something went wrong
        vec![]
    }

    fn decrease_index(&mut self, _amount: usize) {
        unreachable!();
    }
}
