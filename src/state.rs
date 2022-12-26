use crate::context::Context;
use crate::error::Error as StateError;
use crate::option::StateOption;
use crate::serde_state::SerDeState;
use crate::status::Output;
use crate::{
    collection::{Collection, ContextLikeCollection},
    status::{InputStatus, OutputStatus, StatusLike},
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::sync::Arc;

pub enum State {
    OptionsState(OptionsState),
    ContextState(ContextState),
    StateHolder(StateHolder),
}

impl Drop for State {
    fn drop(&mut self) {
        println!("Dropping state: {}", self.get_name());
    }
}

impl State {
    pub fn get_name(&self) -> String {
        match self {
            State::OptionsState(state) => state.get_name(),
            State::ContextState(state) => state.get_name(),
            State::StateHolder(state) => state.get_name(),
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            State::OptionsState(state) => state.get_description(),
            State::ContextState(state) => state.get_description(),
            _ => unimplemented!(),
        }
    }

    pub fn get_parent(&self) -> Option<Arc<RwLock<State>>> {
        match self {
            State::OptionsState(state) => state.get_parent(),
            State::ContextState(state) => state.get_parent(),
            _ => unimplemented!(),
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
            _ => unimplemented!(),
        }
    }

    pub fn get_contexts(&mut self) -> Option<&mut Vec<Context>> {
        match self {
            State::ContextState(state) => state.get_contexts(),
            _ => unimplemented!(),
        }
    }

    pub fn set_options(&mut self, options: Vec<StateOption>) {
        match self {
            State::OptionsState(state) => state.set_options(options),
            _ => unimplemented!(),
        }
    }

    pub fn set_contexts(&mut self, contexts: Vec<Context>) {
        match self {
            State::ContextState(state) => state.set_contexts(contexts),
            _ => unimplemented!(),
        }
    }

    pub fn set_next(&mut self, next: Option<Arc<RwLock<State>>>) {
        match self {
            State::ContextState(state) => state.set_next(next),
            _ => unimplemented!(),
        }
    }

    pub fn decrease_index(&mut self, amount: usize) {
        match self {
            State::ContextState(state) => state.decrease_index(amount),
            _ => {}
        }
    }

    pub fn reset_index(&mut self) {
        match self {
            State::ContextState(state) => state.reset_index(),
            _ => {} //Could reset index of options state. Reset index of the state of every option!
        }
    }

    pub fn input(&mut self, input: String) -> Result<InputStatus, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.input(input),
            State::ContextState(state) => state.input(input),
            _ => unimplemented!(),
        }
    }

    pub fn output(&mut self) -> Result<OutputStatus, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.output(),
            State::ContextState(state) => state.output(),
            _ => unimplemented!(),
        }
    }

    pub fn back(&mut self) -> InputStatus {
        match self {
            State::OptionsState(state) => state.back(),
            State::ContextState(state) => state.back(),
            _ => unimplemented!(),
        }
    }

    pub fn collect(&mut self) -> Result<Result<Vec<Collection>, StateError>, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.collect(),
            State::ContextState(state) => state.collect(),
            _ => unimplemented!(),
        }
    }

    pub fn into_state_sandwich(&mut self) -> Result<Option<Arc<RwLock<State>>>, Box<dyn StdError>> {
        match self {
            State::OptionsState(state) => state.into_state_sandwich(),
            State::ContextState(state) => state.into_state_sandwich(),
            State::StateHolder(state) => state.into_state_sandwich(),
        }
    }

    pub fn destroy(&mut self, destroy_options: bool) {
        match self {
            State::OptionsState(state) => state.destroy(destroy_options),
            State::ContextState(state) => state.destroy(destroy_options),
            State::StateHolder(state) => state.destroy(destroy_options),
        }
    }
}

pub struct StateHolder {
    pub parent: Option<Arc<RwLock<State>>>,
    pub path: String,
    pub how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
    pub which_function: usize,
    pub state: Option<Arc<RwLock<State>>>,
    pub cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>>,
}

impl StateHolder {
    pub fn new(
        parent: Option<Arc<RwLock<State>>>,
        path: String,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
        which_function: usize,
        lazy: bool,
        cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>>,
    ) -> Result<StateHolder, Box<dyn StdError>> {
        println!("Creating state holder for {}", path);
        let mut state_holder = StateHolder {
            parent,
            path,
            how_to_get_string,
            which_function,
            state: None,
            cache,
        };
        if !lazy {
            state_holder.into_state_sandwich()?;
        }
        Ok(state_holder)
    }

    fn into_state_sandwich(&mut self) -> Result<Option<Arc<RwLock<State>>>, Box<dyn StdError>> {
        if self.state.is_some() {
            //dbg!("State already exists");
            return Ok(self.state.clone());
        }

        if let Some(state) = self.cache.read().get(&self.path) {
            //dbg!("State already exists in cache");
            self.state = Some(state.clone());
            return Ok(self.state.clone());
        }

        //dbg!("State creating");
        let function = self
            .how_to_get_string
            .get(self.which_function)
            .ok_or("Function not found")?;
        let string = function(self.path.clone())?;
        let state: SerDeState = serde_yaml::from_str(&string)?;
        let state = state.into_state(
            self.parent.clone(),
            self.how_to_get_string.clone(),
            self.cache.clone(),
        )??;

        self.state = Some(state.clone());
        self.cache.write().insert(self.path.clone(), state.clone());

        Ok(Some(state))
    }

    fn destroy(&mut self, destroy_options: bool) {
        if let Some(state) = &self.state {
            state.write().destroy(destroy_options);
        }
        self.state = None;
        self.cache.write().remove(&self.path);
    }

    fn get_name(&self) -> String {
        self.path.clone()
    }
}

pub struct OptionsState {
    pub name: String,
    pub description: String,
    pub index: usize,
    pub parent: Option<Arc<RwLock<State>>>,
    pub options: Vec<StateOption>,
}

pub struct ContextState {
    pub name: String,
    pub description: String,
    pub index: usize,
    pub parent: Option<Arc<RwLock<State>>>,
    pub next: Option<Arc<RwLock<State>>>,
    pub contexts: Vec<Context>,
    pub submit: bool,
    pub go_back: bool,
}

impl ContextState {
    pub fn new(
        name: String,
        description: String,
        parent: Option<Arc<RwLock<State>>>,
        next: Option<Arc<RwLock<State>>>,
        contexts: Vec<Context>,
        submit: bool,
    ) -> ContextState {
        println!("Creating context state {}", name);
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

    fn destroy(&mut self, destroy_options: bool) {
        for context in &mut self.contexts {
            context.destroy();
        }
        if let Some(next) = &self.next {
            next.write().destroy(destroy_options);
        }
        self.parent = None;
        self.next = None;
    }

    fn on_highest_index(&mut self, status: &mut impl StatusLike) -> Result<(), Box<dyn StdError>> {
        status.set_state_changed(true);
        status.set_submit(self.submit);

        if let Some(next) = &mut self.next {
            dbg!("Next state");
            let nxt = next.write().into_state_sandwich()?;
            if nxt.is_some() {
                status.set_state(nxt);
            } else {
                status.set_state(Some(next.clone()));
            }
            Ok(())
        } else {
            dbg!("No next state");
            status.set_submit(true);
            Ok(())
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_parent(&self) -> Option<Arc<RwLock<State>>> {
        self.parent.clone()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_contexts(&mut self) -> Option<&mut Vec<Context>> {
        Some(&mut self.contexts)
    }

    fn set_contexts(&mut self, contexts: Vec<Context>) {
        self.contexts = contexts;
    }

    fn set_next(&mut self, next: Option<Arc<RwLock<State>>>) {
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
            output: None,
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
                    output: None,
                });
            }
        }

        let output = Output::new(
            self.get_name(),
            self.get_description(),
            vec![self.contexts[self.index].get_name()],
            String::new(),
        );

        Ok(OutputStatus {
            state_changed: false,
            state: None,
            submit: false,
            output: Some(output),
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
            if let Some(p) = &self.parent {
                p.write().decrease_index(1);
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

    fn reset_index(&mut self) {
        self.index = 0;
        if self.next.is_some() {
            self.next.as_ref().unwrap().write().reset_index();
        }
    }

    pub fn into_state_sandwich(&mut self) -> Result<Option<Arc<RwLock<State>>>, Box<dyn StdError>> {
        Ok(None)
    }
}

impl OptionsState {
    pub fn new(
        name: String,
        description: String,
        parent: Option<Arc<RwLock<State>>>,
        options: Vec<StateOption>,
    ) -> OptionsState {
        println!("Creating options state {}", name);
        OptionsState {
            name,
            description,
            index: 0,
            parent,
            options,
        }
    }

    fn destroy(&mut self, destroy_options: bool) {
        for option in &mut self.options {
            option.destroy(destroy_options);
        }
        self.parent = None;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_parent(&self) -> Option<Arc<RwLock<State>>> {
        self.parent.clone()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_options(&mut self) -> Option<&mut Vec<StateOption>> {
        Some(&mut self.options)
    }

    fn set_options(&mut self, options: Vec<StateOption>) {
        self.options = options;
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
        let output = Output::new(
            self.get_name(),
            self.get_description(),
            self.options.iter().map(|x| x.get_name()).collect(),
            String::new(),
        );

        Ok(OutputStatus {
            state_changed: false,
            submit: false,
            state: None,
            output: Some(output),
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

    pub fn into_state_sandwich(&mut self) -> Result<Option<Arc<RwLock<State>>>, Box<dyn StdError>> {
        Ok(None)
    }
}
