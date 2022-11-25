use crate::{BoxDynIntoStateLike, OptionRcRefCellDynStateLike};

pub struct ContextLikeCollection {
    pub name: String,
    pub value: String,
}

impl ContextLikeCollection {
    pub fn new(name: String, value: String) -> ContextLikeCollection {
        ContextLikeCollection { name, value }
    }
}

//TODO use dyn contextLike instead of StateContext
pub trait ContextLike {
    fn input(&mut self, input: String) -> OptionRcRefCellDynStateLike;
    fn output(&mut self) -> OptionRcRefCellDynStateLike;
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
    fn collect(&self) -> ContextLikeCollection;
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
    fn input(&mut self, input: String) -> OptionRcRefCellDynStateLike {
        self.value = input;
        None
    }

    fn output (&mut self) -> OptionRcRefCellDynStateLike {
        None
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&self) -> ContextLikeCollection {
        ContextLikeCollection::new(self.name.clone(), self.value.clone())
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
    fn input(&mut self, input: String) -> OptionRcRefCellDynStateLike {
        self.value = input;
        self.state.into_state_like()
    }

    fn output (&mut self) -> OptionRcRefCellDynStateLike {
        self.state.into_state_like()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&self) -> ContextLikeCollection {
        ContextLikeCollection::new(self.name.clone(), self.value.clone())
    }
}
