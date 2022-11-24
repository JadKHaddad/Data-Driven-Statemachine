use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    context_like::StateContext,
    option_like::StateOption,
    state_like::{ContextState, OptionsState},
    BoxDynIntoStateLike, BoxDynOptionLike, OptionRcRefCellDynStateLike, RcRefCellDynStateLike,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeState {
    pub name: String,
    pub description: String,
    pub r#type: StateType,
}

impl SerDeState {
    pub fn into_state_like(self, parent: OptionRcRefCellDynStateLike) -> RcRefCellDynStateLike {
        let state: RcRefCellDynStateLike = match self.r#type {
            StateType::Context(contexts, next, submit) => {
                let contexts: Vec<StateContext> =
                    contexts.into_iter().map(|x| x.into_context()).collect();
                let state = Rc::new(RefCell::new(ContextState::new(
                    self.name,
                    self.description,
                    parent,
                    None,
                    contexts,
                    submit,
                )));
                if let Some(next) = next {
                    let next_state = next.into_into_state_like(Some(state.clone()));
                    state.borrow_mut().next = Some(next_state);
                }
                state
            }
            StateType::Options(options) => {
                let state = Rc::new(RefCell::new(OptionsState::new(
                    self.name,
                    self.description,
                    parent,
                    vec![],
                )));
                let options: Vec<BoxDynOptionLike> = options
                    .into_iter()
                    .map(|x| x.into_option_like(Some(state.clone())))
                    .collect();
                state.borrow_mut().options = options;
                state
            }
        };
        state
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
        let state = self.state.into_into_state_like(parent);
        Box::new(StateOption::new(self.name, state, self.submit))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SerDeIntoStateLike {
    Inline(SerDeState),
    Path(String /*path to state*/, bool /*lazy*/),
}

impl SerDeIntoStateLike {
    pub fn into_into_state_like(self, parent: OptionRcRefCellDynStateLike) -> BoxDynIntoStateLike {
        match self {
            SerDeIntoStateLike::Inline(state) => Box::new(state.into_state_like(parent)),
            SerDeIntoStateLike::Path(path, lazy) => {
                //stateholder
                todo!();
            }
        }
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
