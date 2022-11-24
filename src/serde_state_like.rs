use std::{cell::RefCell, rc::Rc, task::Context};

use serde::{Deserialize, Serialize};

use crate::{context_like::StateContext, RcRefCellDynStateLike, state_like::ContextState, BoxDynOptionLike, OptionRcRefCellDynStateLike};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeState {
    pub name: String,
    pub description: String,
    pub r#type: StateType,
}

impl SerDeState {
    pub fn into_state_like(self, parent: OptionRcRefCellDynStateLike) -> RcRefCellDynStateLike {
        let state = match self.r#type {
            StateType::Context(contexts, next, submit) => {
                let contexts: Vec<StateContext> =
                    contexts.into_iter().map(|x| x.into_context()).collect();
                    Box::new(ContextState::new(
                        self.name,
                        self.description,
                        None,
                        None,
                        contexts,
                        submit
                    ))
            }
            StateType::Options(options) => {
                todo!()
            }
        };
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeContext {
    pub name: String,
}

impl SerDeContext {
    pub fn into_context(self) -> StateContext {
        StateContext {
            name: self.name,
            value: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeOption {
    pub name: String,
    pub submit: bool,
    pub state: SerDeIntoStateLike,
}

impl SerDeOption {
    pub fn into_option_like(self, parent: OptionRcRefCellDynStateLike) -> BoxDynOptionLike {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SerDeIntoStateLike {
    Inline(SerDeState),
    Path(String /*path to state*/),
}

impl SerDeIntoStateLike {
    pub fn into_into_state_like(self) -> OptionRcRefCellDynStateLike {
        todo!()
    }
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
