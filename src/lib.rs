use parking_lot::RwLock;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use state_like::ContextState;
use state_like::OptionsState;
use state_like::State;

pub type RcRefCellOptionsState = Rc<RefCell<OptionsState>>;
pub type RcRefCellContextState = Rc<RefCell<ContextState>>;

pub type ArcRwLockOptionsState = Arc<RwLock<OptionsState>>;
pub type ArcRwLockContextState = Arc<RwLock<ContextState>>;

pub type ArcRwLockBoxOptionsState = Arc<RwLock<Box<OptionsState>>>;
pub type ArcRwLockBoxContextState = Arc<RwLock<Box<ContextState>>>;

pub type ArcRwLockState = Arc<RwLock<State>>;
pub type OptionArcRwLockState = Option<ArcRwLockState>;

pub mod collection;
pub mod context_like;
pub mod error;
pub mod option_like;
pub mod serde_state_like;
pub mod state_like;
pub mod status;
