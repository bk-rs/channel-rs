use crate::error::SendErrorWithoutFull;

//
pub trait Sender<T> {
    fn send(self, t: T) -> Result<(), SendErrorWithoutFull<T>>;
}
