use crate::{context_like::ContextLikeCollection};

pub struct Collection {
    pub name: String,
    pub context_collections: Vec<ContextLikeCollection>,
}

impl Collection {
    pub fn new(name: String, context_collections:  Vec<ContextLikeCollection>) -> Collection {
        Collection { name, context_collections }
    }
}
