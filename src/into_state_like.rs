use crate::RcRefCellDynStateLike;

pub trait IntoStateLike {
    fn into_state_like(self) -> RcRefCellDynStateLike;
}