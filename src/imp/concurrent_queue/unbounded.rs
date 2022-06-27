use std::sync::Arc;

pub struct Tx<T>(Arc<concurrent_queue::ConcurrentQueue<T>>);

impl<T> Tx<T> {
    pub fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.0.push(msg).map_err(|err| match err {
            concurrent_queue::PushError::Full(_) => panic!("shall not happen"),
            concurrent_queue::PushError::Closed(msg) => crate::SendError::Disconnected(msg),
        })
    }
}

impl<T> Clone for Tx<T> {
    fn clone(&self) -> Self {
        Tx(Arc::clone(&self.0))
    }
}

pub fn unbounded<T>() -> (Tx<T>, super::Rx<T>) {
    let q = Arc::new(concurrent_queue::ConcurrentQueue::unbounded());
    (Tx(Arc::clone(&q)), super::Rx(Arc::clone(&q)))
}
