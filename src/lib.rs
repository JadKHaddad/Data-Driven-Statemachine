use std::{cell::RefCell, rc::Rc};

use option_like::OptionLike;
use state_like::ContextState;
use state_like::OptionsState;
use state_like::StateLike;

pub type RcRefCellOptionsState = Rc<RefCell<OptionsState>>;
pub type RcRefCellContextState = Rc<RefCell<ContextState>>;
pub type RcRefCellDynStateLike = Rc<RefCell<dyn StateLike>>;
pub type OptionRcRefCellDynStateLike = Option<RcRefCellDynStateLike>;
pub type VecBoxDynOptionLike = Vec<Box<dyn OptionLike>>;
pub type OptionVecBoxDynOptionLike = Option<Vec<Box<dyn OptionLike>>>;

pub mod collection;
pub mod context_like;
pub mod option_like;
pub mod serde_state_like;
pub mod state_like;
pub mod status;
