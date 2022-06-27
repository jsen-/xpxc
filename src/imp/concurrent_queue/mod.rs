pub mod mpsc;
pub mod spmc;

use std::{marker::PhantomData, sync::Arc};

mod bounded;
mod unbounded;

pub use bounded::bounded;
pub use unbounded::unbounded;

pub struct Rx<T>(Arc<concurrent_queue::ConcurrentQueue<T>>);
impl<T> Rx<T> {
    pub fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.0.pop().map_err(|err| match err {
            concurrent_queue::PopError::Empty => crate::TryRecvError::Empty,
            concurrent_queue::PopError::Closed => crate::TryRecvError::Disconnected,
        })
    }
}
impl<T> Clone for Rx<T> {
    fn clone(&self) -> Self {
        Rx(self.0.clone())
    }
}

pub struct Channel<T> {
    _p: PhantomData<T>,
}
