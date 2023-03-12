pub use async_channel::Sender as AsyncChannelSender;
use async_channel::TrySendError;

use crate::{SendError, Sender};

//
impl<T> Sender<T> for AsyncChannelSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.try_send(t).map_err(Into::into)
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
        let (tx, rx) = async_channel::bounded(1);
        let sender: Box<dyn Sender<usize>> = Box::new(tx);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(sender.send(2), Err(SendError::Full(2)));
        assert_eq!(rx.recv().await, Ok(1));
    }

    #[tokio::test]
    async fn test_with_unbounded() {
        let (tx, rx) = async_channel::unbounded();
        let sender: Box<dyn Sender<usize>> = Box::new(tx);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(sender.send(2), Ok(()));
        assert_eq!(rx.recv().await, Ok(1));
        assert_eq!(rx.recv().await, Ok(2));
    }
}
