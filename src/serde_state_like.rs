use crate::error::Error as StateError;
use crate::state_like::StateHolder;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::{cell::RefCell, rc::Rc};

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
    pub fn into_state_like(
        self,
        parent: OptionRcRefCellDynStateLike,
        how_to_get_string: fn(String) -> Result<String, Box<dyn StdError>>,
    ) -> Result<Result<RcRefCellDynStateLike, StateError>, Box<dyn StdError>> {
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
                    .map(|x| x.into_context_like(Some(state.clone()), how_to_get_string))
                    .collect::<Result<Result<Vec<BoxDynContextLike>, StateError>,Box<dyn StdError>>>()??;
                state.borrow_mut().contexts = contexts;

                if let Some(next) = next {
                    let next_state =
                        next.into_into_state_like(Some(state.clone()), how_to_get_string)?;
                    state.borrow_mut().next = Some(next_state?);
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
                    .map(|x| x.into_option_like(Some(state.clone()), None, how_to_get_string))
                    .collect::<Result<Result<Vec<BoxDynOptionLike>, StateError>,Box<dyn StdError>>>()??;
                state.borrow_mut().options = options;
                state
            }
        };
        Ok(Ok(state))
    }

    pub fn create_from_yaml_str(
        how_to_get_string: fn(String) -> Result<String, Box<dyn StdError>>,
        name: String,
    ) -> Result<Result<RcRefCellDynStateLike, StateError>, Box<dyn StdError>> {
        let string = how_to_get_string(name)?;
        let state: SerDeState = serde_yaml::from_str(&string)?;
        Ok(state.into_state_like(None, how_to_get_string)?)
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
    Options(Vec<SerDeOption>, String, String),
}

impl SerDeContext {
    pub fn into_context_like(
        self,
        parent_of_options_state: OptionRcRefCellDynStateLike,
        how_to_get_string: fn(String) -> Result<String, Box<dyn StdError>>,
    ) -> Result<Result<BoxDynContextLike, StateError>, Box<dyn StdError>> {
        let value = self.value.unwrap_or_default();

        match self.r#type {
            ContextType::Normal => {
                return Ok(Ok(Box::new(StateContext {
                    name: self.name,
                    value,
                })));
            }
            ContextType::Options(options, given_option, given_question) => {
                let name = match parent_of_options_state.clone() {
                    Some(parent) => parent.borrow().get_name(),
                    None => String::new(),
                };
                //create the valid options state
                let state_for_valid_options = Rc::new(RefCell::new(OptionsState::new(
                    name.clone(),
                    self.name.clone(),
                    parent_of_options_state.clone(),
                    vec![],
                )));
                //the context will be automatically added to the state
                //create a context state with only one context
                if let Some(some_parent_of_options_state) = parent_of_options_state.clone() {
                    let state_for_context = Rc::new(RefCell::new(ContextState::new(
                        name.clone(),
                        self.name.clone(),
                        Some(state_for_valid_options.clone()),
                        Some(Box::new(some_parent_of_options_state.clone())),
                        vec![Box::new(StateContext {
                            name: given_question,
                            value: String::new(),
                        })],
                        false,
                    )));

                    //create the option that holds the context state
                    let option =
                        StateOption::new(given_option, Box::new(state_for_context.clone()), false);

                    //create the valid options
                    let mut options: Vec<BoxDynOptionLike> =
                        options
                            .into_iter()
                            .map(|x| {
                                x.into_option_like(
                                    Some(state_for_valid_options.clone()),
                                    parent_of_options_state.clone(),
                                    how_to_get_string,
                                )
                            })
                            .collect::<Result<
                                Result<Vec<BoxDynOptionLike>, StateError>,
                                Box<dyn StdError>,
                            >>()??;

                    //add the option that holds the context state
                    options.push(Box::new(option));

                    //add the options
                    state_for_valid_options.borrow_mut().options = options;

                    //return the OptionsContext
                    return Ok(Ok(Box::new(StateOptionsContext {
                        name: self.name,
                        value,
                        state: Box::new(state_for_valid_options),
                    })));
                }
                Ok(Err(StateError::BadConstruction))
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
        how_to_get_string: fn(String) -> Result<String, Box<dyn StdError>>,
    ) -> Result<Result<BoxDynOptionLike, StateError>, Box<dyn StdError>> {
        let submit = self.submit.unwrap_or(false);

        if let Some(state) = self.state {
            let state = state.into_into_state_like(parent, how_to_get_string)??;
            return Ok(Ok(Box::new(StateOption::new(self.name, state, submit))));
        }

        if let Some(rc_refcell_state) = backup_state {
            return Ok(Ok(Box::new(StateOption::new(
                self.name,
                Box::new(rc_refcell_state),
                submit,
            ))));
        }
        Ok(Err(StateError::BadConstruction))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SerDeIntoStateLike {
    Inline(SerDeState),
    Path(String /*path to state*/, Option<bool> /*lazy*/),
}

impl SerDeIntoStateLike {
    pub fn into_into_state_like(
        self,
        parent: OptionRcRefCellDynStateLike,
        how_to_get_string: fn(String) -> Result<String, Box<dyn StdError>>,
    ) -> Result<Result<BoxDynIntoStateLike, StateError>, Box<dyn StdError>> {
        match self {
            SerDeIntoStateLike::Inline(state) => {
                let state: BoxDynIntoStateLike =
                    Box::new(state.into_state_like(parent, how_to_get_string)??);
                Ok(Ok(state))
            }
            SerDeIntoStateLike::Path(path, lazy) => {
                let lazy = lazy.unwrap_or(false);
                let state_holder: BoxDynIntoStateLike = Box::new(StateHolder::new(
                    move || {
                        let string = how_to_get_string(path.clone())?;
                        let state: SerDeState = serde_yaml::from_str(&string)?;
                        Ok(state.into_state_like(parent.clone(), how_to_get_string)??)
                    },
                    lazy,
                )?);
                Ok(Ok(state_holder))
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
