use crate::ArcRwLockBoxDynIntoStateLike;
use crate::error::Error as StateError;
use crate::state_like::StateHolder;
use crate::{
    context_like::{Context, StateContext, StateOptionsContext},
    option_like::StateOption,
    state_like::{ContextState, OptionsState, State},
    ArcRwLockState, BoxDynIntoStateLike,
    OptionArcRwLockState,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerDeState {
    pub name: String,
    pub description: String,
    pub r#type: StateType,
}

impl SerDeState {
    pub fn into_state_like(
        self,
        parent: OptionArcRwLockState,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
    ) -> Result<Result<ArcRwLockState, StateError>, Box<dyn StdError>> {
        let state: ArcRwLockState = match self.r#type {
            StateType::Context(contexts, next, submit) => {
                let state: ArcRwLockState = Arc::new(RwLock::new(State::ContextState(ContextState::new(
                    self.name,
                    self.description,
                    parent,
                    None,
                    vec![],
                    submit,
                ))));

                let contexts: Vec<Context> = contexts
                    .into_iter()
                    .map(|x| x.into_context_like(Some(state.clone()), how_to_get_string.clone()))
                    .collect::<Result<Result<Vec<Context>, StateError>,Box<dyn StdError>>>()??;
                state.write().set_contexts(contexts);

                if let Some(next) = next {
                    let next_state =
                        next.into_into_state_like(Some(state.clone()), how_to_get_string.clone())?;
                    state.write().set_next(Some(next_state?));
                }
                state
            }
            StateType::Options(options) => {
                let state: ArcRwLockState = Arc::new(RwLock::new(State::OptionsState(OptionsState::new(
                    self.name,
                    self.description,
                    parent,
                    vec![],
                ))));
                let options: Vec<StateOption> = options
                    .into_iter()
                    .map(|x| x.into_option_like(Some(state.clone()), None, how_to_get_string.clone()))
                    .collect::<Result<Result<Vec<StateOption>, StateError>,Box<dyn StdError>>>()??;
                state.write().set_options(options);
                state
            }
        };
        Ok(Ok(state))
    }

    pub fn create_from_yaml_str(
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
        name: String,
        which_function: usize,
    ) -> Result<Result<ArcRwLockState, StateError>, Box<dyn StdError>> {
        let function = how_to_get_string
            .get(which_function)
            .ok_or("Function not found")?;
        let string = function(name)?;
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
        parent_of_options_state: OptionArcRwLockState,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
    ) -> Result<Result<Context, StateError>, Box<dyn StdError>> {
        let value = self.value.unwrap_or_default();

        match self.r#type {
            ContextType::Normal => {
                return Ok(Ok(Context::StateContext(StateContext {
                    name: self.name,
                    value,
                })));
            }
            ContextType::Options(options, given_option, given_question) => {
                let name = match parent_of_options_state.clone() {
                    Some(parent) => parent.read().get_name(),
                    None => String::new(),
                };
                //create the valid options state
                let state_for_valid_options: ArcRwLockState = Arc::new(RwLock::new(State::OptionsState(OptionsState::new(
                    name.clone(),
                    self.name.clone(),
                    parent_of_options_state.clone(),
                    vec![],
                ))));
                //the context will be automatically added to the state
                //create a context state with only one context
                if let Some(some_parent_of_options_state) = parent_of_options_state.clone() {
                    let state_for_context: ArcRwLockState = Arc::new(RwLock::new(State::ContextState(ContextState::new(
                        name.clone(),
                        self.name.clone(),
                        Some(state_for_valid_options.clone()),
                        Some(Box::new(some_parent_of_options_state.clone())),
                        vec![Context::StateContext(StateContext {
                            name: given_question,
                            value: String::new(),
                        })],
                        false,
                    ))));
                    
                    //create the option that holds the context state
                    let option =
                        StateOption::new(given_option, Box::new(state_for_context), false);

                    //create the valid options
                    let mut options: Vec<StateOption> =
                        options
                            .into_iter()
                            .map(|x| {
                                x.into_option_like(
                                    Some(state_for_valid_options.clone()),
                                    parent_of_options_state.clone(),
                                    how_to_get_string.clone(),
                                )
                            })
                            .collect::<Result<
                                Result<Vec<StateOption>, StateError>,
                                Box<dyn StdError>,
                            >>()??;

                    //add the option that holds the context state
                    options.push(option);

                    //add the options
                    state_for_valid_options.write().set_options(options);

                    //return the OptionsContext
                    return Ok(Ok(Context::StateOptionsContext(StateOptionsContext {
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
        parent: OptionArcRwLockState,
        backup_state: OptionArcRwLockState,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
    ) -> Result<Result<StateOption, StateError>, Box<dyn StdError>> {
        let submit = self.submit.unwrap_or(false);

        if let Some(state) = self.state {
            let state = state.into_into_state_like(parent, how_to_get_string)??;
            return Ok(Ok(StateOption::new(self.name, state, submit)));
        }

        if let Some(rc_refcell_state) = backup_state {
            return Ok(Ok(StateOption::new(
                self.name,
                Box::new(rc_refcell_state),
                submit,
            )));
        }
        Ok(Err(StateError::BadConstruction))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SerDeIntoStateLike {
    Inline(SerDeState),
    Path(
        String,       /*path to state*/
        Option<bool>, /*lazy*/
        usize,        /*which function*/
    ),
}

impl SerDeIntoStateLike {
    pub fn into_into_state_like(
        self,
        parent: OptionArcRwLockState,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
    ) -> Result<Result<BoxDynIntoStateLike, StateError>, Box<dyn StdError>> {
        match self {
            SerDeIntoStateLike::Inline(state) => {
                let state: BoxDynIntoStateLike =
                    Box::new(state.into_state_like(parent, how_to_get_string)??);
                Ok(Ok(state))
            }
            SerDeIntoStateLike::Path(path, lazy, which_function) => {
                let lazy = lazy.unwrap_or(false);
                let state_holder: BoxDynIntoStateLike = Box::new(StateHolder::new(
                    move || {
                        let function = how_to_get_string
                            .get(which_function)
                            .ok_or("Function not found")?;
                        let string = function(path.clone())?;
                        let state: SerDeState = serde_yaml::from_str(&string)?;
                        Ok(state.into_state_like(parent.clone(), how_to_get_string.clone())??)
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
