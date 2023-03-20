use crate::error::{OneshotRecvError, TryRecvError};

//
#[async_trait::async_trait]
pub trait AsyncReceiver<T> {
    async fn recv(self) -> Result<T, OneshotRecvError>
    where
        T: Send;

    fn try_recv(&mut self) -> Result<T, TryRecvError>;
}
