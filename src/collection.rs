

#[derive(Debug)]
pub struct Collection {
    pub state_name: String,
    pub context_collections: Vec<ContextLikeCollection>,
}

impl Collection {
    pub fn new(name: String, context_collections:  Vec<ContextLikeCollection>) -> Collection {
        Collection { state_name: name, context_collections }
    }
}
#[derive(Debug)]
pub struct ContextLikeCollection {
    pub name: String,
    pub value: String,
}

impl ContextLikeCollection {
    pub fn new(name: String, value: String) -> ContextLikeCollection {
        ContextLikeCollection { name, value }
    }
}
