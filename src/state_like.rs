use parking_lot::RwLock;

use crate::context_like::Context;
use crate::error::Error as StateError;
use crate::option_like::StateOption;
use crate::{
    collection::{Collection, ContextLikeCollection},
    status::{InputStatus, OutputStatus, StatusLike},
    ArcRwLockState, OptionArcRwLockState,
};
use std::error::Error as StdError;
use std::sync::Arc;

#[derive(Clone)]
pub enum State {
    OptionsState(OptionsState),
    ContextState(ContextState),
}

impl State {
    pub fn get_name(&self) -> String {
        match self {
            State::OptionsState(state) => state.get_name(),
            State::ContextState(state) => state.get_name(),
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            State::OptionsState(state) => state.get_description(),
            State::ContextState(state) => state.get_description(),
        }
    }

    pub fn get_parent(&self) -> OptionArcRwLockState {
        match self {
            State::OptionsState(state) => state.get_parent(),
            State::ContextState(state) => state.get_parent(),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            State::OptionsState(state) => state.get_index(),
            State::ContextState(state) => state.get_index(),
            _ => unimplemented!(),
        }
    }

    pub fn get_options(&mut self) -> Option<&mut Vec<StateOption>> {
        match self {
            State::OptionsState(state) => state.get_options(),
            State::ContextState(state) => state.get_options(),
        }
    }

    pub fn get_contexts(&mut self) -> Option<&mut Vec<Context>> {
        match self {
            State::OptionsState(state) => state.get_contexts(),
            State::ContextState(state) => state.get_contexts(),
        }
    }

    pub fn set_options(&mut self, options: Vec<StateOption>) {
        match self {
            State::OptionsState(state) => state.set_options(options),
            State::ContextState(state) => state.set_options(options),
        }
    }

    pub fn set_contexts(&mut self, contexts: Vec<Context>) {
        match self {
            State::OptionsState(state) => state.set_contexts(contexts),
            State::ContextState(state) => state.set_contexts(contexts),
        }
    }

    pub fn set_next(&mut self, next: Option<Arc<RwLock<IntoState>>>) {
        match self {
            State::OptionsState(state) => state.set_next(next),
            State::ContextState(state) => state.set_next(next),
        }
    }

    pub fn decrease_index(&mut self, amount: usize) {
        match self {
            State::OptionsState(state) => state.decrease_index(amount),
            State::ContextState(state) => state.decrease_index(amount),
        }
    }

    pub fn input(&mut self, input: String) -> Result<InputStatus, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.input(input),
            State::ContextState(state) => state.input(input),
        }
    }

    pub fn output(&mut self) -> Result<OutputStatus, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.output(),
            State::ContextState(state) => state.output(),
        }
    }

    pub fn back(&mut self) -> InputStatus {
        match self {
            State::OptionsState(state) => state.back(),
            State::ContextState(state) => state.back(),
        }
    }

    pub fn collect(&mut self) -> Result<Result<Vec<Collection>, StateError>, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.collect(),
            State::ContextState(state) => state.collect(),
        }
    }

    pub fn into_into_state(&self) -> IntoState {
        match self {
            State::OptionsState(state) => IntoState::OptionsState(state.clone()),
            State::ContextState(state) => IntoState::ContextState(state.clone()),
        }
    }
}

pub enum IntoState {
    OptionsState(OptionsState),
    ContextState(ContextState),
    StateHolder(StateHolder),
}

impl IntoState {
    pub fn into_state_like(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        match self {
            IntoState::OptionsState(state) => state.into_state_like(),
            IntoState::ContextState(state) => state.into_state_like(),
            IntoState::StateHolder(state) => state.into_state_like(),
        }
    }
}

pub struct StateHolder {
    pub closure_state: Box<dyn Fn() -> Result<ArcRwLockState, Box<dyn StdError>>>, //parent should be passed in
    pub state: OptionArcRwLockState,
}

impl StateHolder {
    pub fn new(
        closure_state: impl Fn() -> Result<ArcRwLockState, Box<dyn StdError>> + 'static,
        lazy: bool,
    ) -> Result<StateHolder, Box<dyn StdError>> {
        let mut state_holder = StateHolder {
            closure_state: Box::new(closure_state),
            state: None,
        };
        if !lazy {
            state_holder.into_state_like()?;
        }
        Ok(state_holder)
    }

    fn into_state_like(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        if let Some(state) = &self.state {
            dbg!("State already exists");
            return Ok(Some(state.clone()));
        }
        dbg!("State creating");
        let state = (self.closure_state)()?;
        self.state = Some(state.clone());
        Ok(Some(state))
    }
}

#[derive(Clone)]
pub struct OptionsState {
    pub name: String,
    pub description: String,
    pub index: usize,
    pub parent: OptionArcRwLockState,
    pub options: Vec<StateOption>,
}

impl OptionsState {
    pub fn new(
        name: String,
        description: String,
        parent: OptionArcRwLockState,
        options: Vec<StateOption>,
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

#[derive(Clone)]
pub struct ContextState {
    pub name: String,
    pub description: String,
    pub index: usize,
    pub parent: OptionArcRwLockState,
    pub next: Option<Arc<RwLock<IntoState>>>,
    pub contexts: Vec<Context>,
    pub submit: bool,
    pub go_back: bool,
}

impl ContextState {
    pub fn new(
        name: String,
        description: String,
        parent: OptionArcRwLockState,
        next: Option<Arc<RwLock<IntoState>>>,
        contexts: Vec<Context>,
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

    fn on_highest_index(&mut self, status: &mut impl StatusLike) -> Result<(), Box<dyn StdError>> {
        status.set_state_changed(true);
        status.set_submit(self.submit);

        if let Some(next) = &mut self.next {
            dbg!("Next state");
            status.set_state(next.write().into_state_like()?);
            Ok(())
        } else {
            dbg!("No next state");
            status.set_submit(true);
            Ok(())
        }
    }
}

impl ContextState {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_parent(&self) -> OptionArcRwLockState {
        self.parent.clone()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_options(&mut self) -> Option<&mut Vec<StateOption>> {
        None
    }

    fn get_contexts(&mut self) -> Option<&mut Vec<Context>> {
        Some(&mut self.contexts)
    }

    fn set_options(&mut self, options: Vec<StateOption>) {
        //do nothing
    }

    fn set_contexts(&mut self, contexts: Vec<Context>) {
        self.contexts = contexts;
    }

    fn set_next(&mut self, next: Option<Arc<RwLock<IntoState>>>) {
        self.next = next;
    }

    fn input(&mut self, input: String) -> Result<InputStatus, Box<dyn StdError>> {
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
            self.on_highest_index(&mut status)?;
        }

        return Ok(status);
    }

    fn output(&mut self) -> Result<OutputStatus, Box<dyn StdError>> {
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
            return Ok(status);
        }

        if self.index >= self.contexts.len() {
            self.on_highest_index(&mut status)?;
            return Ok(status);
        }

        //this means that the current context is an option
        if let Some(context) = self.contexts.get_mut(self.index) {
            let next_state = context.output()?;
            if next_state.is_some() {
                if self.index < self.contexts.len() {
                    self.index += 1;
                }
                return Ok(OutputStatus {
                    state_changed: true,
                    state: next_state,
                    submit: false,
                    output: String::new(),
                });
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
        Ok(OutputStatus {
            state_changed: false,
            state: None,
            submit: false,
            output,
        })
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

    fn collect(&mut self) -> Result<Result<Vec<Collection>, StateError>, Box<dyn StdError>> {
        let collection = Collection {
            state_name: self.get_name(),
            context_collections: self
                .contexts
                .iter_mut()
                .map(|context| context.collect())
                .collect::<Result<
                Result<Vec<ContextLikeCollection>, StateError>,
                Box<dyn StdError>,
            >>()??,
        };

        if let Some(parent) = &self.parent {
            let mut parent_collections = parent.write().collect()??;
            parent_collections.push(collection);
            return Ok(Ok(parent_collections));
        }

        Ok(Ok(vec![collection]))
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

    pub fn into_state_like(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        let state = Arc::new(RwLock::new(State::ContextState(self.clone())));
        Ok(Some(state))
    }
}

impl OptionsState {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_parent(&self) -> OptionArcRwLockState {
        self.parent.clone()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_options(&mut self) -> Option<&mut Vec<StateOption>> {
        Some(&mut self.options)
    }

    fn get_contexts(&mut self) -> Option<&mut Vec<Context>> {
        None
    }

    fn set_options(&mut self, options: Vec<StateOption>) {
        self.options = options;
    }

    fn set_contexts(&mut self, _contexts: Vec<Context>) {
        //do nothing
    }

    fn set_next(&mut self, next: Option<Arc<RwLock<IntoState>>>) {
        //do nothing
    }

    fn input(&mut self, input: String) -> Result<InputStatus, Box<dyn StdError>> {
        let mut status = InputStatus {
            state_changed: false,
            state: None,
            submit: false,
            input_recognized: false,
        };

        fn on_input_recognized(
            status: &mut InputStatus,
            option: &mut StateOption,
        ) -> Result<(), Box<dyn StdError>> {
            status.state_changed = true;
            status.state = option.get_state()?;
            status.submit = option.get_submit();
            status.input_recognized = true;
            Ok(())
        }

        if let Ok(input_as_u32) = input.parse::<u32>() {
            if input_as_u32 > 0 {
                let index = input_as_u32 as usize - 1;
                if let Some(option) = self.options.get_mut(index) {
                    on_input_recognized(&mut status, option)?;
                    self.index = index;
                    return Ok(status);
                }
            }
        }
        for (index, option) in self.options.iter_mut().enumerate() {
            if option.input(&input) {
                on_input_recognized(&mut status, option)?;
                self.index = index;
                return Ok(status);
            }
        }

        return Ok(status);
    }

    fn output(&mut self) -> Result<OutputStatus, Box<dyn StdError>> {
        let mut output = format!("[{}]\n", self.name);
        output.push_str(&format!("{}\n", self.description));
        for (index, option) in self.options.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", index + 1, option.get_name()));
        }
        Ok(OutputStatus {
            output,
            state_changed: false,
            submit: false,
            state: None,
        })
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
            parent.write().decrease_index(2);
        }

        return status;
    }

    fn collect(&mut self) -> Result<Result<Vec<Collection>, StateError>, Box<dyn StdError>> {
        if let Some(option) = self.options.get(self.index) {
            let context_like_collection =
                ContextLikeCollection::new(self.name.clone(), option.get_name());

            let mut collection = Collection {
                state_name: "None".to_string(),
                context_collections: vec![context_like_collection],
            };

            if let Some(parent) = &self.parent {
                let mut parent_mute = parent.write();
                collection.state_name = parent_mute.get_name();
                let mut parent_collections = parent_mute.collect()??;
                parent_collections.push(collection);
                return Ok(Ok(parent_collections));
            }

            return Ok(Ok(vec![collection]));
        }
        //something went wrong
        Ok(Err(StateError::BadConstruction))
    }

    fn decrease_index(&mut self, _amount: usize) {
        unreachable!();
    }

    pub fn into_state_like(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        let state = Arc::new(RwLock::new(State::OptionsState(self.clone())));
        Ok(Some(state))
    }
}
