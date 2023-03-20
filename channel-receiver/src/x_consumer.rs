use dyn_clone::{clone_trait_object, DynClone};

use crate::error::TryRecvError;

//
#[async_trait::async_trait]
pub trait AsyncReceiver<T> {
    async fn recv(&mut self) -> Option<T>
    where
        T: Send;

    fn try_recv(&mut self) -> Result<T, TryRecvError>;
}

#[async_trait::async_trait]
pub trait CloneableAsyncReceiver<T>: DynClone {
    async fn recv(&self) -> Option<T>
    where
        T: Send;

    fn try_recv(&self) -> Result<T, TryRecvError>;
}
clone_trait_object!(<T> CloneableAsyncReceiver<T>);
