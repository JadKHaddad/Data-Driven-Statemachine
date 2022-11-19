use crate::context_like::StateContext;

pub struct Collection {
    pub name: String,
    pub contexts: Vec<StateContext>,
}

impl Collection {
    pub fn new(name: String, contexts: Vec<StateContext>) -> Collection {
        Collection { name, contexts }
    }
}
