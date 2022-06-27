use std::sync::Arc;

pub struct Tx<T>(Arc<concurrent_queue::ConcurrentQueue<T>>);
impl<T> Tx<T> {
    pub fn try_send(&mut self, msg: T) -> Result<(), crate::TrySendError<T>> {
        self.0.push(msg).map_err(|err| match err {
            concurrent_queue::PushError::Closed(msg) => crate::TrySendError::Disconnected(msg),
            concurrent_queue::PushError::Full(msg) => crate::TrySendError::Full(msg),
        })
    }
}
impl<T> Clone for Tx<T> {
    fn clone(&self) -> Self {
        Tx(Arc::clone(&self.0))
    }
}

pub fn bounded<T>(capacity: usize) -> (Tx<T>, super::Rx<T>) {
    let q = Arc::new(concurrent_queue::ConcurrentQueue::bounded(capacity));
    (Tx(Arc::clone(&q)), super::Rx(Arc::clone(&q)))
}
