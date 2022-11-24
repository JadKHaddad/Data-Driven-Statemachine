//TODO use dyn contextLike instead of StateContext
pub trait ContextLike {
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
    fn set_value(&mut self, value: String);
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
