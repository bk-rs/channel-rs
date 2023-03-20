//
#[derive(Debug, Eq)]
pub enum TryRecvError {
    Empty,
    Closed,
    Disconnected,
}
impl core::fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for TryRecvError {}
impl core::cmp::PartialEq for TryRecvError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Empty, Self::Empty)
                | (Self::Closed, Self::Closed)
                | (Self::Closed, Self::Disconnected)
                | (Self::Disconnected, Self::Disconnected)
                | (Self::Disconnected, Self::Closed)
        )
    }
}

impl TryRecvError {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_closed_or_disconnected(&self) -> bool {
        matches!(self, Self::Closed | Self::Disconnected)
    }
}

//
#[derive(Debug, PartialEq, Eq)]
pub enum OneshotRecvError {
    Dropped,
}
impl core::fmt::Display for OneshotRecvError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for OneshotRecvError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_recv_error_partial_eq() {
        assert_eq!(TryRecvError::Empty, TryRecvError::Empty);
        assert_eq!(TryRecvError::Closed, TryRecvError::Closed);
        assert_eq!(TryRecvError::Closed, TryRecvError::Disconnected);
        assert_eq!(TryRecvError::Disconnected, TryRecvError::Disconnected);
        assert_ne!(TryRecvError::Empty, TryRecvError::Closed);
        assert_ne!(TryRecvError::Empty, TryRecvError::Disconnected);
    }
}
