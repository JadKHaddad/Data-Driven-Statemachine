use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateCreator {
    pub name: String,
    pub description: String,
    pub r#type: StateType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContextCreator {
    pub name: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OptionCreator {
    pub name: String,
    pub submit: bool,
    pub r#type: OptionType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OptionType {
    State(StateCreator),
    Closure(String /*path to state*/),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StateType {
    Options(Vec<OptionCreator>),
    Context(Vec<ContextCreator> /*context*/, Option<Box<StateCreator>> /*next state*/, bool /*submit*/),
}

