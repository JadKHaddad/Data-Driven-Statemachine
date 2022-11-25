use crate::{BoxDynIntoStateLike, OptionRcRefCellDynStateLike, collection::Collection};

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
    fn input(&mut self, input: String);
    fn output(&mut self) -> OptionRcRefCellDynStateLike;
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
    fn collect(&mut self) -> Vec<Collection>;
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
    fn input(&mut self, input: String){
        self.value = input;
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

    fn collect(&mut self) -> Vec<Collection> {
        vec![Collection::new(self.name.clone(), vec![ContextLikeCollection::new(self.name.clone(), self.value.clone())])]
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

    fn output (&mut self) -> OptionRcRefCellDynStateLike {
        self.state.into_state_like()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn collect(&mut self) -> Vec<Collection> {
        let collection = Collection::new(self.name.clone(), vec![ContextLikeCollection::new(self.name.clone(), self.value.clone())]);
        if let Some(state) = self.state.into_state_like() {
            let mut collections = state.borrow_mut().collect_contexts();
            collections.push(collection);
            return collections;
        }
        //let collections = self.state.into_state_like().borrow_mut().collect().context_collections();
        vec![collection]
    }
}
