use parking_lot::RwLock;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use context_like::ContextLike;
use option_like::OptionLike;
use state_like::ContextState;
use state_like::IntoStateLike;
use state_like::OptionsState;
use state_like::StateLike;

pub type RcRefCellOptionsState = Rc<RefCell<OptionsState>>;
pub type RcRefCellContextState = Rc<RefCell<ContextState>>;

pub type ArcRwLockOptionsState = Arc<RwLock<OptionsState>>;
pub type ArcRwLockContextState = Arc<RwLock<ContextState>>;

pub type RcRefCellDynStateLike = Rc<RefCell<dyn StateLike>>;
pub type OptionRcRefCellDynStateLike = Option<RcRefCellDynStateLike>;

pub type ArcRwLockDynStateLike = Arc<RwLock<dyn StateLike>>;
pub type OptionArcRwLockDynStateLike = Option<ArcRwLockDynStateLike>;

pub type RcRefCellDynIntoStateLike = Rc<RefCell<dyn IntoStateLike>>;
pub type OptionRcRefCellDynIntoStateLike = Option<RcRefCellDynIntoStateLike>;

pub type ArcRwLockDynIntoStateLike = Arc<RwLock<dyn IntoStateLike>>;
pub type OptionArcRwLockDynIntoStateLike = Option<ArcRwLockDynIntoStateLike>;

pub type BoxDynIntoStateLike = Box<dyn IntoStateLike>;
pub type OptionBoxDynIntoStateLike = Option<BoxDynIntoStateLike>;

pub type BoxDynOptionLike = Box<dyn OptionLike>;
pub type VecBoxDynOptionLike = Vec<BoxDynOptionLike>;
pub type OptionVecBoxDynOptionLike = Option<VecBoxDynOptionLike>;

pub type BoxDynContextLike = Box<dyn ContextLike>;
pub type VecBoxDynContextLike = Vec<BoxDynContextLike>;
pub type OptionVecBoxDynContextLike = Option<VecBoxDynContextLike>;

pub mod collection;
pub mod context_like;
pub mod error;
pub mod option_like;
pub mod serde_state_like;
pub mod state_like;
pub mod status;
