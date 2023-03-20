use std::sync::mpsc::TrySendError;
pub use std::sync::mpsc::{Sender as StdSender, SyncSender as StdSyncSender};

//
mod multi_producer_impl {
    use super::*;

    use crate::{error::SendErrorWithoutFull, multi_producer::UnboundedSender};

    impl<T> UnboundedSender<T> for StdSender<T> {
        fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>> {
            StdSender::send(self, t).map_err(|err| SendErrorWithoutFull::Disconnected(err.0))
        }
    }
}

//
mod generic_impl {
    use super::*;

    use crate::{
        error::SendError,
        generic::{CloneableSender, Sender},
    };

    impl<T> Sender<T> for StdSender<T> {
        fn send(&self, t: T) -> Result<(), SendError<T>> {
            // https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html
            StdSender::send(self, t).map_err(|err| SendError::Disconnected(err.0))
        }
    }

    impl<T> Sender<T> for StdSyncSender<T> {
        fn send(&self, t: T) -> Result<(), SendError<T>> {
            StdSyncSender::try_send(self, t).map_err(Into::into)
        }
    }

    //
    impl<T> CloneableSender<T> for StdSender<T> {
        fn send(&self, t: T) -> Result<(), SendError<T>> {
            // https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html
            StdSender::send(self, t).map_err(|err| SendError::Disconnected(err.0))
        }
    }

    impl<T> CloneableSender<T> for StdSyncSender<T> {
        fn send(&self, t: T) -> Result<(), SendError<T>> {
            StdSyncSender::try_send(self, t).map_err(Into::into)
        }
    }
}

//
mod error_convert {
    use super::*;

    use crate::error::SendError;

    impl<T> From<TrySendError<T>> for SendError<T> {
        fn from(err: TrySendError<T>) -> Self {
            match err {
                TrySendError::Full(v) => Self::Full(v),
                TrySendError::Disconnected(v) => Self::Disconnected(v),
            }
        }
    }
}

#[cfg(test)]
mod multi_producer_impl_tests {
    use crate::{error::SendErrorWithoutFull, multi_producer::UnboundedSender};

    #[test]
    fn test_with_channel() {
        {
            let (tx, rx) = std::sync::mpsc::channel();
            let sender: Box<dyn UnboundedSender<usize>> = Box::new(tx);
            let sender = sender.clone();
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv(), Ok(1));
            assert_eq!(rx.recv(), Ok(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendErrorWithoutFull::Disconnected(3)));
        }
    }
}

#[cfg(test)]
mod generic_impl_tests {
    use crate::{
        error::SendError,
        generic::{CloneableSender, Sender},
    };

    #[test]
    fn test_with_channel() {
        {
            let (tx, rx) = std::sync::mpsc::channel();
            let sender: Box<dyn Sender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv(), Ok(1));
            assert_eq!(rx.recv(), Ok(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Disconnected(3)));
        }
        {
            let (tx, rx) = std::sync::mpsc::channel();
            let sender: Box<dyn CloneableSender<usize>> = Box::new(tx);
            let sender = sender.clone();
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Ok(()));
            assert_eq!(rx.recv(), Ok(1));
            assert_eq!(rx.recv(), Ok(2));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Disconnected(3)));
        }
    }

    #[test]
    fn test_with_sync_channel() {
        {
            let (tx, rx) = std::sync::mpsc::sync_channel(1);
            let sender: Box<dyn Sender<usize>> = Box::new(tx);
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Err(SendError::Full(2)));
            assert_eq!(rx.recv(), Ok(1));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Disconnected(3)));
        }
        {
            let (tx, rx) = std::sync::mpsc::sync_channel(1);
            let sender: Box<dyn CloneableSender<usize>> = Box::new(tx);
            let sender = sender.clone();
            assert_eq!(sender.send(1), Ok(()));
            assert_eq!(sender.send(2), Err(SendError::Full(2)));
            assert_eq!(rx.recv(), Ok(1));
            drop(rx);
            assert_eq!(sender.send(3), Err(SendError::Disconnected(3)));
        }
    }
}
