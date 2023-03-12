use tokio::sync::mpsc::error::TrySendError;
pub use tokio::sync::mpsc::{Sender as TokioSender, UnboundedSender as TokioUnboundedSender};

use crate::{SendError, Sender};

//
impl<T> Sender<T> for TokioSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.try_send(t).map_err(Into::into)
    }
}

impl<T> Sender<T> for TokioUnboundedSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        TokioUnboundedSender::send(self, t).map_err(|err| SendError::Closed(err.0))
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
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let sender: Box<dyn Sender<usize>> = Box::new(tx);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(sender.send(2), Err(SendError::Full(2)));
        assert_eq!(rx.recv().await, Some(1));
    }

    #[tokio::test]
    async fn test_with_unbounded_channel() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let sender: Box<dyn Sender<usize>> = Box::new(tx);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(sender.send(2), Ok(()));
        assert_eq!(rx.recv().await, Some(1));
        assert_eq!(rx.recv().await, Some(2));
    }
}
