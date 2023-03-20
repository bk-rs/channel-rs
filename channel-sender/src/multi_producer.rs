use dyn_clone::{clone_trait_object, DynClone};

use crate::error::{SendError, SendErrorWithoutFull};

//
#[async_trait::async_trait]
pub trait BoundedSender<T>: DynClone {
    async fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>>
    where
        T: Send;

    fn try_send(&self, t: T) -> Result<(), SendError<T>>;
}
clone_trait_object!(<T> BoundedSender<T>);

pub trait UnboundedSender<T>: DynClone {
    fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>>;
}
clone_trait_object!(<T> UnboundedSender<T>);
