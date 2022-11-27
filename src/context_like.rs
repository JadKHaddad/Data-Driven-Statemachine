use crate::{collection::ContextLikeCollection, BoxDynIntoStateLike, OptionRcRefCellDynStateLike};
use crate::error::Error as StateError;

pub trait ContextLike {
    fn input(&mut self, input: String);
    fn output(&mut self) -> OptionRcRefCellDynStateLike;
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
    fn collect(&mut self) -> Result<ContextLikeCollection, StateError>;
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
}

impl ContextLike for StateContext {
    fn input(&mut self, input: String) {
        self.value = input;
    }

    fn output(&mut self) -> OptionRcRefCellDynStateLike {
        None
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&mut self) -> Result<ContextLikeCollection, StateError> {
        Ok(ContextLikeCollection::new(self.name.clone(), self.value.clone()))
    }
}

pub struct StateOptionsContext {
    //this context will give an OptionsState when it is triggered.
    //the OptionsState has StateOptions and one special StateOption.
    //the state in the StateOptions would be the parent of the state that has this context.
    //the state in the special StateOption is a ContextState with the parent of the OptionsState with only one context.
    //the next state of the ContextState would be the parent of the state that has this option
    pub name: String,
    pub value: String,
    pub state: BoxDynIntoStateLike,
}

impl StateOptionsContext {
    pub fn new(name: String, value: String, state: BoxDynIntoStateLike) -> StateOptionsContext {
        StateOptionsContext { name, value, state }
    }
}

impl ContextLike for StateOptionsContext {
    fn input(&mut self, input: String) {
        self.value = input;
    }

    fn output(&mut self) -> OptionRcRefCellDynStateLike {
        self.state.into_state_like()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&mut self) -> Result<ContextLikeCollection, StateError> {
        if let Some(state) = self.state.into_state_like() {
            let mut state = state.borrow_mut();
            let index = state.get_index();
            let options = state.get_options();
            if let Some(options) = options {
                let len = options.len();
                if let Some(option) = options.get_mut(index) {
                    if index == len - 1 {
                        let in_state = option.get_state();
                        if let Some(in_state) = in_state {
                            let mut in_state = in_state.borrow_mut();
                            let contexts = in_state.get_contexts();
                            if let Some(contexts) = contexts {
                                let context = contexts.get_mut(0);
                                if let Some(context) = context {
                                    return Ok(ContextLikeCollection::new(
                                        self.name.clone(),
                                        context.get_value(),
                                    ));
                                }
                            }
                        }
                    }
                    return Ok(ContextLikeCollection::new(self.name.clone(), option.get_name()));
                }
            }
        }
        //something went wrong
        Err(StateError::BadConstruction)
    }
}
