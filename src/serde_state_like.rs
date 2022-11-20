use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeState {
    pub name: String,
    pub description: String,
    pub r#type: StateType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeContext {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeOption {
    pub name: String,
    pub submit: bool,
    pub state: SerDeIntoStateLike,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SerDeIntoStateLike {
    Inline(SerDeState),
    Path(String /*path to state*/),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StateType {
    Options(Vec<SerDeOption>),
    Context(
        Vec<SerDeContext>,               /*context*/
        Option<Box<SerDeIntoStateLike>>, /*next state*/
        bool,                            /*submit*/
    ),
}
