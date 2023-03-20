use dyn_clone::{clone_trait_object, DynClone};

use crate::error::SendError;

//
pub trait Sender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>>;
}

pub trait CloneableSender<T>: DynClone {
    fn send(&self, t: T) -> Result<(), SendError<T>>;
}
clone_trait_object!(<T> CloneableSender<T>);
