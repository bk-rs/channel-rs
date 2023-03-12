use tokio::sync::mpsc::error::TrySendError;
pub use tokio::sync::mpsc::{Sender as TokioSender, UnboundedSender as TokioUnboundedSender};

use crate::{BoundedSender, SendError, SendErrorWithoutFull, Sender, UnboundedSender};

//
impl<T> Sender<T> for TokioSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        TokioSender::try_send(self, t).map_err(Into::into)
    }
}

#[async_trait::async_trait]
impl<T> BoundedSender<T> for TokioSender<T> {
    async fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>>
    where
        T: Send,
    {
        TokioSender::send(self, t)
            .await
            .map_err(|err| SendErrorWithoutFull::Closed(err.0))
    }

    fn try_send(&self, t: T) -> Result<(), SendError<T>> {
        TokioSender::try_send(self, t).map_err(Into::into)
    }
}

impl<T> Sender<T> for TokioUnboundedSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        TokioUnboundedSender::send(self, t).map_err(|err| SendError::Closed(err.0))
    }
}

impl<T> UnboundedSender<T> for TokioUnboundedSender<T> {
    fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>> {
        TokioUnboundedSender::send(self, t).map_err(|err| SendErrorWithoutFull::Closed(err.0))
    }
}

//
impl<T> From<TrySendError<T>> for SendError<T> {
    fn from(err: TrySendError<T>) -> Self {
        match err {
            TrySendError::Full(v) => Self::Full(v),
            TrySendError::Closed(v) => Self::Closed(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_channel() {
        {
            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            let sender: Box<dyn Sender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Err(SendError::Full(2)));
            assert_eq!(rx.recv().await, Some(1));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Closed(3)));
        }

        {
            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            let sender: Box<dyn BoundedSender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1).await, Ok(()));
            assert_eq!(sender.try_send(2), Err(SendError::Full(2)));
            assert!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), sender.send(2))
                    .await
                    .is_err()
            );
            assert_eq!(rx.recv().await, Some(1));
            drop(rx);
            assert_eq!(sender.send(3).await, Err(SendErrorWithoutFull::Closed(3)));
            assert_eq!(sender.try_send(3), Err(SendError::Closed(3)));
        }
    }

    #[tokio::test]
    async fn test_with_unbounded_channel() {
        {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let sender: Box<dyn Sender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv().await, Some(1));
            assert_eq!(rx.recv().await, Some(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Closed(3)));
        }

        {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let sender: Box<dyn UnboundedSender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv().await, Some(1));
            assert_eq!(rx.recv().await, Some(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendErrorWithoutFull::Closed(3)));
        }
    }
}
