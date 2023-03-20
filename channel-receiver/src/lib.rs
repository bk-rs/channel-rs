//
pub mod multi_consumer;
pub use multi_consumer as mc;

pub mod single_consumer;
pub use single_consumer as sc;

pub mod one_shot;
pub use one_shot as oneshot;

pub mod generic;

pub mod error;
pub use error::TryRecvError;

//
#[cfg(feature = "impl_async_channel")]
pub mod impl_async_channel;
#[cfg(feature = "impl_tokio")]
pub mod impl_tokio;
