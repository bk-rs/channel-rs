use std::sync::mpsc::TrySendError;
pub use std::sync::mpsc::{Sender as StdSender, SyncSender as StdSyncSender};

use crate::{SendError, SendErrorWithoutFull, Sender, UnboundedSender};

//
impl<T> Sender<T> for StdSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        // https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html
        StdSender::send(self, t).map_err(|err| SendError::Disconnected(err.0))
    }
}

impl<T> Sender<T> for StdSyncSender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.try_send(t).map_err(Into::into)
    }
}

//
impl<T> From<TrySendError<T>> for SendError<T> {
    fn from(err: TrySendError<T>) -> Self {
        match err {
            TrySendError::Full(v) => Self::Full(v),
            TrySendError::Disconnected(v) => Self::Disconnected(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_channel() {
        let (tx, rx) = std::sync::mpsc::channel();
        let sender: Box<dyn Sender<usize>> = Box::new(tx);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(sender.send(2), Ok(()));
        assert_eq!(rx.recv(), Ok(1));
        assert_eq!(rx.recv(), Ok(2));
    }

    #[test]
    fn test_with_sync_channel() {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        let sender: Box<dyn Sender<usize>> = Box::new(tx);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(sender.send(2), Err(SendError::Full(2)));
        assert_eq!(rx.recv(), Ok(1));
    }
}
