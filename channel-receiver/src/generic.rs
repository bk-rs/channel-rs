use crate::error::TryRecvError;

//
#[async_trait::async_trait]
pub trait AsyncReceiver<T> {
    async fn recv(&mut self) -> Option<T>
    where
        T: Send;

    fn try_recv(&mut self) -> Result<T, TryRecvError>;
}
