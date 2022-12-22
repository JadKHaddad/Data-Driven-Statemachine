use crate::error::Error as StateError;
use crate::{collection::ContextLikeCollection, OptionArcRwLockState, State};
use std::error::Error as StdError;

#[derive(Clone)]
pub enum Context {
    StateContext(StateContext),
    StateOptionsContext(StateOptionsContext),
}

impl Context {
    pub fn input(&mut self, input: String) {
        match self {
            Context::StateContext(state_context) => state_context.input(input),
            Context::StateOptionsContext(state_options_context) => {
                state_options_context.input(input)
            }
        }
    }
    pub fn output(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        match self {
            Context::StateContext(state_context) => state_context.output(),
            Context::StateOptionsContext(state_options_context) => state_options_context.output(),
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            Context::StateContext(state_context) => state_context.get_name(),
            Context::StateOptionsContext(state_options_context) => state_options_context.get_name(),
        }
    }
    pub fn get_value(&self) -> String {
        match self {
            Context::StateContext(state_context) => state_context.get_value(),
            Context::StateOptionsContext(state_options_context) => {
                state_options_context.get_value()
            }
        }
    }
    pub fn collect(
        &mut self,
    ) -> Result<Result<ContextLikeCollection, StateError>, Box<dyn StdError>> {
        match self {
            Context::StateContext(state_context) => state_context.collect(),
            Context::StateOptionsContext(state_options_context) => state_options_context.collect(),
        }
    }
}

#[derive(Clone)]
pub struct StateContext {
    pub name: String,
    pub value: String,
}

impl StateContext {
    pub fn new(name: String, value: String) -> StateContext {
        StateContext { name, value }
    }
    fn input(&mut self, input: String) {
        self.value = input;
    }

    fn output(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        Ok(None)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&mut self) -> Result<Result<ContextLikeCollection, StateError>, Box<dyn StdError>> {
        Ok(Ok(ContextLikeCollection::new(
            self.name.clone(),
            self.value.clone(),
        )))
    }
}

#[derive(Clone)]
pub struct StateOptionsContext {
    //this context will give an OptionsState when it is triggered.
    //the OptionsState has StateOptions and one special StateOption.
    //the state in the StateOptions would be the parent of the state that has this context.
    //the state in the special StateOption is a ContextState with the parent of the OptionsState with only one context.
    //the next state of the ContextState would be the parent of the state that has this option
    pub name: String,
    pub value: String,
    pub state: State,
}

impl StateOptionsContext {
    pub fn new(name: String, value: String, state: State) -> StateOptionsContext {
        StateOptionsContext { name, value, state }
    }
    fn input(&mut self, input: String) {
        self.value = input;
    }

    fn output(&mut self) -> Result<OptionArcRwLockState, Box<dyn StdError>> {
        self.state.into_state_like()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&mut self) -> Result<Result<ContextLikeCollection, StateError>, Box<dyn StdError>> {
        if let Some(state) = self.state.into_state_like()? {
            let mut state = state.write();
            let index = state.get_index();
            let options = state.get_options();
            if let Some(options) = options {
                let len = options.len();
                if let Some(option) = options.get_mut(index) {
                    if index == len - 1 {
                        let in_state = option.get_state()?;
                        if let Some(in_state) = in_state {
                            let mut in_state = in_state.write();
                            let contexts = in_state.get_contexts();
                            if let Some(contexts) = contexts {
                                let context = contexts.get_mut(0);
                                if let Some(context) = context {
                                    return Ok(Ok(ContextLikeCollection::new(
                                        self.name.clone(),
                                        context.get_value(),
                                    )));
                                }
                            }
                        }
                    }
                    return Ok(Ok(ContextLikeCollection::new(
                        self.name.clone(),
                        option.get_name(),
                    )));
                }
            }
        }
        //something went wrong
        Ok(Err(StateError::BadConstruction))
    }
}
