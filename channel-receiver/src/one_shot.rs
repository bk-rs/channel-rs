use core::future::Future;

use crate::error::{OneshotRecvError, TryRecvError};

//
pub trait AsyncReceiver<T>: Future<Output = Result<T, OneshotRecvError>> {
    fn try_recv(&mut self) -> Result<T, TryRecvError>;
}
