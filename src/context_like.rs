//TODO use dyn contextLike instead of StateContext
pub trait ContextLike {
    //TODO
    // fn get_name(&self) -> String;
    // fn get_value(&self) -> String;
    // fn set_value(&mut self, value: String);
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
    //TODO
}

pub struct StateOptionsContext { //this context will give a StateContextOption when it is triggered
    //TODO
}

impl StateOptionsContext {
    //TODO
}

impl ContextLike for StateOptionsContext {
    //TODO
}