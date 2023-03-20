use tokio::sync::{
    mpsc::error::TryRecvError as MpscTryRecvErrorInner,
    oneshot::error::TryRecvError as OneshotTryRecvError,
};
pub use tokio::sync::{
    mpsc::{Receiver as TokioMpscReceiver, UnboundedReceiver as TokioMpscUnboundedReceiver},
    oneshot::Receiver as TokioOneshotReceiver,
};

//
mod x_consumer_impl {
    use super::*;

    use crate::{error::TryRecvError, x_consumer::AsyncReceiver};

    #[async_trait::async_trait]
    impl<T> AsyncReceiver<T> for TokioMpscReceiver<T> {
        async fn recv(&mut self) -> Option<T>
        where
            T: Send,
        {
            TokioMpscReceiver::recv(self).await
        }

        fn try_recv(&mut self) -> Result<T, TryRecvError> {
            TokioMpscReceiver::try_recv(self).map_err(Into::into)
        }
    }

    #[async_trait::async_trait]
    impl<T> AsyncReceiver<T> for TokioMpscUnboundedReceiver<T> {
        async fn recv(&mut self) -> Option<T>
        where
            T: Send,
        {
            TokioMpscUnboundedReceiver::recv(self).await
        }

        fn try_recv(&mut self) -> Result<T, TryRecvError> {
            TokioMpscUnboundedReceiver::try_recv(self).map_err(Into::into)
        }
    }
}

//
mod one_shot_impl {
    use super::*;

    use crate::{
        error::{OneshotRecvError, TryRecvError},
        one_shot::AsyncReceiver,
    };

    #[async_trait::async_trait]
    impl<T> AsyncReceiver<T> for TokioOneshotReceiver<T> {
        async fn recv(self) -> Result<T, OneshotRecvError>
        where
            T: Send,
        {
            self.await.map_err(|_| OneshotRecvError::Dropped)
        }

        fn try_recv(&mut self) -> Result<T, TryRecvError> {
            TokioOneshotReceiver::try_recv(self).map_err(Into::into)
        }
    }
}

//
mod error_convert {
    use super::*;

    use crate::error::TryRecvError;

    impl From<MpscTryRecvErrorInner> for TryRecvError {
        fn from(err: MpscTryRecvErrorInner) -> Self {
            match err {
                MpscTryRecvErrorInner::Empty => Self::Empty,
                MpscTryRecvErrorInner::Disconnected => Self::Disconnected,
            }
        }
    }

    impl From<OneshotTryRecvError> for TryRecvError {
        fn from(err: OneshotTryRecvError) -> Self {
            match err {
                OneshotTryRecvError::Empty => Self::Empty,
                OneshotTryRecvError::Closed => Self::Closed,
            }
        }
    }
}

#[cfg(test)]
mod x_consumer_impl_tests {
    use crate::{error::TryRecvError, x_consumer::AsyncReceiver};

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
