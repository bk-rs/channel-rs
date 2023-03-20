//
#[derive(Debug, Eq)]
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
impl<T: core::cmp::PartialEq> core::cmp::PartialEq for SendError<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Full(v1), Self::Full(v2)) => v1 == v2,
            (Self::Closed(v1), Self::Closed(v2)) | (Self::Closed(v1), Self::Disconnected(v2)) => {
                v1 == v2
            }
            (Self::Disconnected(v1), Self::Disconnected(v2))
            | (Self::Disconnected(v1), Self::Closed(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl<T> SendError<T> {
    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full(_))
    }

    pub fn is_closed_or_disconnected(&self) -> bool {
        matches!(self, Self::Closed(_) | Self::Disconnected(_))
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

//
#[derive(Debug, Eq)]
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
impl<T: core::cmp::PartialEq> core::cmp::PartialEq for SendErrorWithoutFull<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Closed(v1), Self::Closed(v2)) | (Self::Closed(v1), Self::Disconnected(v2)) => {
                v1 == v2
            }
            (Self::Disconnected(v1), Self::Disconnected(v2))
            | (Self::Disconnected(v1), Self::Closed(v2)) => v1 == v2,
            (Self::UnreachableFull(v1), Self::UnreachableFull(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl<T> SendErrorWithoutFull<T> {
    pub fn is_closed_or_disconnected(&self) -> bool {
        matches!(self, Self::Closed(_) | Self::Disconnected(_))
    }

    pub fn is_unreachable_full(&self) -> bool {
        matches!(self, Self::UnreachableFull(_))
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_error_partial_eq() {
        assert_eq!(SendError::Full(1), SendError::Full(1));
        assert_eq!(SendError::Closed(1), SendError::Closed(1));
        assert_eq!(SendError::Closed(1), SendError::Disconnected(1));
        assert_eq!(SendError::Disconnected(1), SendError::Disconnected(1));
        assert_ne!(SendError::Full(1), SendError::Closed(1));
        assert_ne!(SendError::Full(1), SendError::Disconnected(1));
    }

    #[test]
    fn test_send_error_without_full_partial_eq() {
        assert_eq!(
            SendErrorWithoutFull::UnreachableFull(1),
            SendErrorWithoutFull::UnreachableFull(1)
        );
        assert_eq!(
            SendErrorWithoutFull::Closed(1),
            SendErrorWithoutFull::Closed(1)
        );
        assert_eq!(
            SendErrorWithoutFull::Closed(1),
            SendErrorWithoutFull::Disconnected(1)
        );
        assert_eq!(
            SendErrorWithoutFull::Disconnected(1),
            SendErrorWithoutFull::Disconnected(1)
        );
        assert_ne!(
            SendErrorWithoutFull::UnreachableFull(1),
            SendErrorWithoutFull::Closed(1)
        );
        assert_ne!(
            SendErrorWithoutFull::UnreachableFull(1),
            SendErrorWithoutFull::Disconnected(1)
        );
    }
}
