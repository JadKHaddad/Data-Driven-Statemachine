use crate::{
    context::{Context, StateContext, StateOptionsContext},
    error::Error as StateError,
    option::StateOption,
    state::StateHolder,
    state::{ContextState, OptionsState, State},
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error as StdError, sync::Arc};

#[derive(Debug, Deserialize, Serialize)]
pub struct SerDeState {
    pub name: String,
    pub description: String,
    pub r#type: StateType,
}

impl SerDeState {
    pub fn into_state(
        self,
        parent: Option<Arc<RwLock<State>>>,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
        cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>>,
    ) -> Result<Result<Arc<RwLock<State>>, StateError>, Box<dyn StdError>> {
        let state: Arc<RwLock<State>> = match self.r#type {
            StateType::Context(contexts, submit, next) => {
                let state: Arc<RwLock<State>> = Arc::new(RwLock::new(State::ContextState(
                    ContextState::new(self.name, self.description, parent, None, vec![], submit),
                )));

                let contexts: Vec<Context> = contexts
                    .into_iter()
                    .map(|x| {
                        x.into_context(
                            Some(state.clone()),
                            how_to_get_string.clone(),
                            cache.clone(),
                        )
                    })
                    .collect::<Result<Result<Vec<Context>, StateError>, Box<dyn StdError>>>()??;
                state.write().set_contexts(contexts);

                if let Some(next) = next {
                    let next_state = next.into_into_state(
                        Some(state.clone()),
                        how_to_get_string.clone(),
                        cache.clone(),
                    )?;
                    state.write().set_next(Some(next_state?));
                }
                state
            }
            StateType::Options(options) => {
                let state: Arc<RwLock<State>> = Arc::new(RwLock::new(State::OptionsState(
                    OptionsState::new(self.name, self.description, parent, vec![]),
                )));
                let options: Vec<StateOption> = options
                    .into_iter()
                    .map(|x| {
                        x.into_option(
                            Some(state.clone()),
                            None,
                            how_to_get_string.clone(),
                            cache.clone(),
                        )
                    })
                    .collect::<Result<Result<Vec<StateOption>, StateError>, Box<dyn StdError>>>(
                    )??;
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
    ) -> Result<Result<Arc<RwLock<State>>, StateError>, Box<dyn StdError>> {
        let cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let function = how_to_get_string
            .get(which_function)
            .ok_or("Function not found")?;
        let string = function(name)?;
        let state: SerDeState = serde_yaml::from_str(&string)?;
        Ok(state.into_state(None, how_to_get_string, cache)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerDeContext {
    pub name: String,
    pub value: Option<String>,
    pub r#type: ContextType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ContextType {
    Normal,
    Options(Vec<SerDeOption>, String, String),
}

impl SerDeContext {
    pub fn into_context(
        self,
        parent_of_options_state: Option<Arc<RwLock<State>>>,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
        cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>>,
    ) -> Result<Result<Context, StateError>, Box<dyn StdError>> {
        let value = self.value.unwrap_or_default();

        match self.r#type {
            ContextType::Normal => {
                return Ok(Ok(Context::StateContext(StateContext::new(
                    self.name, value,
                ))));
            }
            ContextType::Options(options, given_option, given_question) => {
                let name = match parent_of_options_state.clone() {
                    Some(parent) => parent.read().get_name(),
                    None => String::new(),
                };
                //create the valid options state
                let state_for_valid_options: Arc<RwLock<State>> =
                    Arc::new(RwLock::new(State::OptionsState(OptionsState::new(
                        name.clone(),
                        self.name.clone(),
                        parent_of_options_state.clone(),
                        vec![],
                    ))));
                //the context will be automatically added to the state
                //create a context state with only one context
                if let Some(some_parent_of_options_state) = parent_of_options_state.clone() {
                    let state_for_context: Arc<RwLock<State>> =
                        Arc::new(RwLock::new(State::ContextState(ContextState::new(
                            name.clone(),
                            self.name.clone(),
                            Some(state_for_valid_options.clone()),
                            Some(some_parent_of_options_state.clone()),
                            vec![Context::StateContext(StateContext::new(
                                given_question,
                                String::new(),
                            ))],
                            false,
                        ))));

                    //create the option that holds the context state
                    let option = StateOption::new(
                        given_option,
                        Some(state_for_context.clone()),
                        false,
                        false,
                    );

                    //create the valid options
                    let mut options: Vec<StateOption> = options
                        .into_iter()
                        .map(|x| {
                            x.into_option(
                                Some(state_for_valid_options.clone()),
                                parent_of_options_state.clone(),
                                how_to_get_string.clone(),
                                cache.clone(),
                            )
                        })
                        .collect::<Result<Result<Vec<StateOption>, StateError>, Box<dyn StdError>>>(
                        )??;

                    //add the option that holds the context state
                    options.push(option);

                    //add the options
                    state_for_valid_options.write().set_options(options);

                    //return the OptionsContext
                    return Ok(Ok(Context::StateOptionsContext(StateOptionsContext::new(
                        self.name,
                        value,
                        state_for_valid_options.clone(),
                    ))));
                }
                Ok(Err(StateError::BadConstruction))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerDeOption {
    pub name: String,
    pub submit: Option<bool>,
    pub state: Option<SerDeIntoState>,
    pub reset: Option<bool>,
}

impl SerDeOption {
    pub fn into_option(
        self,
        parent: Option<Arc<RwLock<State>>>,
        backup_state: Option<Arc<RwLock<State>>>,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
        cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>>,
    ) -> Result<Result<StateOption, StateError>, Box<dyn StdError>> {
        let submit = self.submit.unwrap_or(false);
        let reset = self.reset.unwrap_or(false);

        if let Some(state) = self.state {
            let state = state.into_into_state(parent, how_to_get_string, cache)??;
            return Ok(Ok(StateOption::new(self.name, Some(state), submit, reset)));
        }

        if let Some(state_g) = backup_state {
            return Ok(Ok(StateOption::new(
                self.name,
                Some(state_g.clone()),
                submit,
                reset,
            )));
        }

        return Ok(Ok(StateOption::new(self.name, None, submit, reset)));
        //Ok(Err(StateError::BadConstruction))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SerDeIntoState {
    Inline(SerDeState),
    Path(
        String,       /*path to state*/
        Option<bool>, /*lazy*/
        usize,        /*which function*/
    ),
}

impl SerDeIntoState {
    pub fn into_into_state(
        self,
        parent: Option<Arc<RwLock<State>>>,
        how_to_get_string: Vec<fn(String) -> Result<String, Box<dyn StdError>>>,
        cache: Arc<RwLock<HashMap<String, Arc<RwLock<State>>>>>,
    ) -> Result<Result<Arc<RwLock<State>>, StateError>, Box<dyn StdError>> {
        match self {
            SerDeIntoState::Inline(state) => {
                let state = state.into_state(parent, how_to_get_string, cache)??;
                Ok(Ok(state))
            }
            SerDeIntoState::Path(path, lazy, which_function) => {
                let lazy = lazy.unwrap_or(false);
                let state_holder = State::StateHolder(StateHolder::new(
                    parent,
                    path,
                    how_to_get_string,
                    which_function,
                    lazy,
                    cache,
                )?);
                Ok(Ok(Arc::new(RwLock::new(state_holder))))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StateType {
    Options(Vec<SerDeOption>),
    Context(
        Vec<SerDeContext>,           /*context*/
        bool,                        /*submit*/
        Option<Box<SerDeIntoState>>, /*next state*/
    ),
}
