use parking_lot::RwLock;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use context_like::Context;
use option_like::StateOption;
use state_like::ContextState;
use state_like::IntoStateLike;
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

pub type RcRefCellDynIntoStateLike = Rc<RefCell<dyn IntoStateLike>>;
pub type OptionRcRefCellDynIntoStateLike = Option<RcRefCellDynIntoStateLike>;

pub type ArcRwLockBoxDynIntoStateLike = Arc<RwLock<Box<dyn IntoStateLike>>>;
pub type OptionArcRwLockBoxDynIntoStateLike = Option<ArcRwLockBoxDynIntoStateLike>;

pub type BoxDynIntoStateLike = Box<dyn IntoStateLike>;
pub type OptionBoxDynIntoStateLike = Option<BoxDynIntoStateLike>;


pub mod collection;
pub mod context_like;
pub mod error;
pub mod option_like;
pub mod serde_state_like;
pub mod state_like;
pub mod status;
