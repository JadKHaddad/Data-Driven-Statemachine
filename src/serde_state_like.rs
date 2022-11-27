use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    context_like::{StateContext, StateOptionsContext},
    option_like::StateOption,
    state_like::{ContextState, OptionsState},
    BoxDynContextLike, BoxDynIntoStateLike, BoxDynOptionLike, OptionRcRefCellDynStateLike,
    RcRefCellDynStateLike, VecBoxDynContextLike,
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
                let state = Rc::new(RefCell::new(ContextState::new(
                    self.name,
                    self.description,
                    parent,
                    None,
                    vec![],
                    submit,
                )));

                let contexts: VecBoxDynContextLike = contexts
                    .into_iter()
                    .map(|x| x.into_context_like(Some(state.clone())))
                    .collect();

                state.borrow_mut().contexts = contexts;

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
                    .map(|x| x.into_option_like(Some(state.clone()), None))
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
    pub value: Option<String>,
    pub r#type: ContextType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ContextType {
    Normal,
    Options(Vec<SerDeOption>),
}

impl SerDeContext {
    pub fn into_context_like(
        self,
        parent_of_options_state: OptionRcRefCellDynStateLike,
    ) -> BoxDynContextLike {
        let value = self.value.unwrap_or_default();

        match self.r#type {
            ContextType::Normal => {
                return Box::new(StateContext {
                    name: self.name,
                    value,
                });
            }
            ContextType::Options(options) => {
                //crteate the valid options state
                let state_for_valid_options = Rc::new(RefCell::new(OptionsState::new(
                    String::from("valid options"),
                    String::from("valid options"),
                    parent_of_options_state.clone(),
                    vec![],
                )));
                //the context will be automatically added to the state
                //create a context state with only one context
                if let Some(some_parent_of_options_state) = parent_of_options_state.clone() {
                    let state_for_context = Rc::new(RefCell::new(ContextState::new(
                        String::from("others"),
                        String::from("others"),
                        Some(state_for_valid_options.clone()),
                        Some(Box::new(some_parent_of_options_state.clone())),
                        vec![Box::new(StateContext {
                            name: String::from("so tell me what you want"),
                            value: String::new(),
                        })],
                        false,
                    )));
                    
                    //create the option that holds the context state
                    let option = StateOption::new(
                        String::from("others"),
                        Box::new(state_for_context.clone()),
                        false,
                    );

                    //create the valid options
                    let mut options: Vec<BoxDynOptionLike> = options
                        .into_iter()
                        .map(|x| {
                            x.into_option_like(
                                Some(state_for_valid_options.clone()),
                                parent_of_options_state.clone(),
                            )
                        })
                        .collect();

                    //add the option that holds the context state
                    options.push(Box::new(option));

                    //add the options
                    state_for_valid_options.borrow_mut().options = options;

                    //return the OptionsContext
                    return Box::new(StateOptionsContext {
                        name: self.name,
                        value,
                        state: Box::new(state_for_valid_options),
                    });
                }
                unreachable!();
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeOption {
    pub name: String,
    pub submit: Option<bool>,
    pub state: Option<SerDeIntoStateLike>,
}

impl SerDeOption {
    pub fn into_option_like(
        self,
        parent: OptionRcRefCellDynStateLike,
        backup_state: OptionRcRefCellDynStateLike,
    ) -> BoxDynOptionLike {
        let submit = self.submit.unwrap_or(false);

        if let Some(state) = self.state {
            let state = state.into_into_state_like(parent);
            return Box::new(StateOption::new(self.name, state, submit));
        }

        if let Some(rc_refcell_state) = backup_state {
            return Box::new(StateOption::new(
                self.name,
                Box::new(rc_refcell_state),
                submit,
            ));
        }

        unreachable!();
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
