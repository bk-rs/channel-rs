//
//
//
pub trait Sender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>>;
}

#[async_trait::async_trait]
pub trait BoundedSender<T> {
    async fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>>
    where
        T: Send;

    fn try_send(&self, t: T) -> Result<(), SendError<T>>;
}

pub trait UnboundedSender<T> {
    fn send(&self, t: T) -> Result<(), SendErrorWithoutFull<T>>;
}

//
//
//
#[derive(Debug, PartialEq, Eq)]
pub enum SendError<T> {
    Full(T),
    Closed(T),
    Disconnected(T),
}
impl<T: core::fmt::Debug> core::fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl<T: core::fmt::Debug> std::error::Error for SendError<T> {}

impl<T> SendError<T> {
    pub fn is_full(&self) -> bool {
        matches!(self, SendError::Full(_))
    }

    pub fn is_closed_or_disconnected(&self) -> bool {
        matches!(self, SendError::Closed(_) | SendError::Disconnected(_))
    }

    pub fn inner(&self) -> &T {
        match &self {
            Self::Full(v) => v,
            Self::Closed(v) => v,
            Self::Disconnected(v) => v,
        }
    }
    pub fn into_inner(self) -> T {
        match self {
            Self::Full(v) => v,
            Self::Closed(v) => v,
            Self::Disconnected(v) => v,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SendErrorWithoutFull<T> {
    Closed(T),
    Disconnected(T),
    UnreachableFull(T),
}
impl<T: core::fmt::Debug> core::fmt::Display for SendErrorWithoutFull<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl<T: core::fmt::Debug> std::error::Error for SendErrorWithoutFull<T> {}

impl<T> SendErrorWithoutFull<T> {
    pub fn inner(&self) -> &T {
        match &self {
            Self::Closed(v) => v,
            Self::Disconnected(v) => v,
            Self::UnreachableFull(v) => v,
        }
    }
    pub fn into_inner(self) -> T {
        match self {
            Self::Closed(v) => v,
            Self::Disconnected(v) => v,
            Self::UnreachableFull(v) => v,
        }
    }
}

//
//
//
#[cfg(feature = "impl_async_channel")]
pub mod impl_async_channel;
#[cfg(feature = "impl_tokio")]
pub mod impl_tokio;

pub mod impl_std;
