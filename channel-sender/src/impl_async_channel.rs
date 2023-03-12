pub use async_channel::Sender as AsyncChannelSender;
use async_channel::TrySendError;

use crate::{BoundedSender, SendError, SendErrorWithoutFull, Sender, UnboundedSender};

//
impl<T> Sender<T> for AsyncChannelSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.try_send(t).map_err(Into::into)
    }
}

#[async_trait::async_trait]
impl<T> BoundedSender<T> for AsyncChannelSender<T> {
    async fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>>
    where
        T: Send,
    {
        self.send(t)
            .await
            .map_err(|err| SendErrorWithoutFull::Closed(err.0))
    }

    fn try_send(&self, t: T) -> Result<(), SendError<T>> {
        self.try_send(t).map_err(Into::into)
    }
}

impl<T> UnboundedSender<T> for AsyncChannelSender<T> {
    fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>> {
        match self.try_send(t) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                TrySendError::Full(v) => Err(SendErrorWithoutFull::UnreachableFull(v)),
                TrySendError::Closed(v) => Err(SendErrorWithoutFull::Closed(v)),
            },
        }
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
    async fn test_with_bounded() {
        {
            let (tx, rx) = async_channel::bounded(1);
            let sender: Box<dyn Sender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Err(SendError::Full(2)));
            assert_eq!(rx.recv().await, Ok(1));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Closed(3)));
        }

        {
            let (tx, rx) = async_channel::bounded(1);
            let sender: Box<dyn BoundedSender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1).await, Ok(()));
            assert_eq!(sender.try_send(2), Err(SendError::Full(2)));
            assert!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), sender.send(2))
                    .await
                    .is_err()
            );
            assert_eq!(rx.recv().await, Ok(1));
            drop(rx);
            assert_eq!(sender.send(3).await, Err(SendErrorWithoutFull::Closed(3)));
        }
    }

    #[tokio::test]
    async fn test_with_unbounded() {
        {
            let (tx, rx) = async_channel::unbounded();
            let sender: Box<dyn Sender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv().await, Ok(1));
            assert_eq!(rx.recv().await, Ok(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Closed(3)));
        }

        {
            let (tx, rx) = async_channel::unbounded();
            let sender: Box<dyn UnboundedSender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv().await, Ok(1));
            assert_eq!(rx.recv().await, Ok(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendErrorWithoutFull::Closed(3)));
        }
    }
}
