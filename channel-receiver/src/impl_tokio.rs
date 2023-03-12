use tokio::sync::mpsc::error::TryRecvError as TryRecvErrorInner;
pub use tokio::sync::mpsc::{
    Receiver as TokioReceiver, UnboundedReceiver as TokioUnboundedReceiver,
};

use crate::{AsyncReceiver, TryRecvError};

//
#[async_trait::async_trait]
impl<T> AsyncReceiver<T> for TokioReceiver<T> {
    async fn recv(&mut self) -> Option<T>
    where
        T: Send,
    {
        TokioReceiver::recv(self).await
    }

    fn try_recv(&mut self) -> Result<T, TryRecvError> {
        TokioReceiver::try_recv(self).map_err(Into::into)
    }

    fn close(&mut self) {
        TokioReceiver::close(self);
    }
}

#[async_trait::async_trait]
impl<T> AsyncReceiver<T> for TokioUnboundedReceiver<T> {
    async fn recv(&mut self) -> Option<T>
    where
        T: Send,
    {
        TokioUnboundedReceiver::recv(self).await
    }

    fn try_recv(&mut self) -> Result<T, TryRecvError> {
        TokioUnboundedReceiver::try_recv(self).map_err(Into::into)
    }

    fn close(&mut self) {
        TokioUnboundedReceiver::close(self);
    }
}

//
impl From<TryRecvErrorInner> for TryRecvError {
    fn from(err: TryRecvErrorInner) -> Self {
        match err {
            TryRecvErrorInner::Empty => Self::Empty,
            TryRecvErrorInner::Disconnected => Self::Disconnected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_channel() {
        {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let mut receiver: Box<dyn AsyncReceiver<usize>> = Box::new(rx);
            assert!(tx.send(1).await.is_ok());
            assert_eq!(receiver.recv().await, Some(1));
            assert_eq!(tx.try_send(2), Ok(()));
            assert_eq!(receiver.recv().await, Some(2));
            assert!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await
                    .is_err()
            );
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
            drop(tx);
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));
            assert_eq!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await,
                Ok(None)
            );
        }
    }

    #[tokio::test]
    async fn test_with_unbounded_channel() {
        {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let mut receiver: Box<dyn AsyncReceiver<usize>> = Box::new(rx);
            assert!(tx.send(1).is_ok());
            assert!(tx.send(2).is_ok());
            assert_eq!(receiver.recv().await, Some(1));
            assert_eq!(receiver.recv().await, Some(2));
            assert!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await
                    .is_err()
            );
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
            drop(tx);
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));
            assert_eq!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await,
                Ok(None)
            );
        }
    }
}
