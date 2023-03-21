use crate::error::SendErrorWithoutFull;

//
pub trait Sender<T> {
    fn send(self, t: T) -> Result<(), SendErrorWithoutFull<T>>;
}

pub trait BoxSender<T> {
    fn send(self: Box<Self>, t: T) -> Result<(), SendErrorWithoutFull<T>>;
}
