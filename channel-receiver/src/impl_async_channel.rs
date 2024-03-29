pub use async_channel::Receiver as AsyncChannelReceiver;
use async_channel::TryRecvError as TryRecvErrorInner;

//
mod multi_consumer_impl {
    use super::*;

    use crate::{error::TryRecvError, multi_consumer::AsyncReceiver};

    #[async_trait::async_trait]
    impl<T> AsyncReceiver<T> for AsyncChannelReceiver<T> {
        async fn recv(&mut self) -> Option<T>
        where
            T: Send,
        {
            AsyncChannelReceiver::recv(self).await.ok()
        }

        fn try_recv(&mut self) -> Result<T, TryRecvError> {
            AsyncChannelReceiver::try_recv(self).map_err(Into::into)
        }
    }
}

//
mod generic_impl {
    use super::*;

    use crate::{error::TryRecvError, generic::AsyncReceiver};

    #[async_trait::async_trait]
    impl<T> AsyncReceiver<T> for AsyncChannelReceiver<T> {
        async fn recv(&mut self) -> Option<T>
        where
            T: Send,
        {
            AsyncChannelReceiver::recv(self).await.ok()
        }

        fn try_recv(&mut self) -> Result<T, TryRecvError> {
            AsyncChannelReceiver::try_recv(self).map_err(Into::into)
        }
    }
}

//
mod error_convert {
    use super::*;

    use crate::error::TryRecvError;

    impl From<TryRecvErrorInner> for TryRecvError {
        fn from(err: TryRecvErrorInner) -> Self {
            match err {
                TryRecvErrorInner::Empty => Self::Empty,
                TryRecvErrorInner::Closed => Self::Closed,
            }
        }
    }
}

#[cfg(test)]
mod multi_consumer_impl_tests {
    use crate::{error::TryRecvError, multi_consumer::AsyncReceiver};

    #[tokio::test]
    async fn test_with_bounded() {
        {
            let (tx, rx) = async_channel::bounded(1);
            let receiver: Box<dyn AsyncReceiver<usize>> = Box::new(rx);
            let mut receiver = receiver.clone();
            assert_eq!(tx.send(1).await, Ok(()));
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
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Closed));
            assert_eq!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await,
                Ok(None)
            );
        }
    }

    #[tokio::test]
    async fn test_with_unbounded() {
        {
            let (tx, rx) = async_channel::unbounded();
            let receiver: Box<dyn AsyncReceiver<usize>> = Box::new(rx);
            let mut receiver = receiver.clone();
            assert_eq!(tx.send(1).await, Ok(()));
            assert_eq!(tx.send(2).await, Ok(()));
            assert_eq!(receiver.recv().await, Some(1));
            assert_eq!(receiver.recv().await, Some(2));
            assert!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await
                    .is_err()
            );
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
            drop(tx);
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Closed));
            assert_eq!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await,
                Ok(None)
            );
        }
    }
}

#[cfg(test)]
mod generic_impl_tests {
    use crate::{error::TryRecvError, generic::AsyncReceiver};

    #[tokio::test]
    async fn test_with_bounded() {
        {
            let (tx, rx) = async_channel::bounded(1);
            let mut receiver: Box<dyn AsyncReceiver<usize>> = Box::new(rx);
            assert_eq!(tx.send(1).await, Ok(()));
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
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Closed));
            assert_eq!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await,
                Ok(None)
            );
        }
    }

    #[tokio::test]
    async fn test_with_unbounded() {
        {
            let (tx, rx) = async_channel::unbounded();
            let mut receiver: Box<dyn AsyncReceiver<usize>> = Box::new(rx);
            assert_eq!(tx.send(1).await, Ok(()));
            assert_eq!(tx.send(2).await, Ok(()));
            assert_eq!(receiver.recv().await, Some(1));
            assert_eq!(receiver.recv().await, Some(2));
            assert!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await
                    .is_err()
            );
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
            drop(tx);
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Closed));
            assert_eq!(
                tokio::time::timeout(tokio::time::Duration::from_millis(200), receiver.recv())
                    .await,
                Ok(None)
            );
        }
    }
}
