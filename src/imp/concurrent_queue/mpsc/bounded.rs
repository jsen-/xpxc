use crate::imp::concurrent_queue as imp;

pub type Channel<T> = imp::Channel<T>;
pub type Tx<T> = imp::bounded::Tx<T>;
pub type Rx<T> = imp::Rx<T>;

impl<T> crate::mpsc::bounded::TrySender<T> for Tx<T> {
    fn try_send(&mut self, msg: T) -> Result<(), crate::TrySendError<T>> {
        self.try_send(msg)
    }
}
impl<T> crate::mpsc::bounded::TryReceiver<T> for Rx<T> {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.try_recv()
    }
}

impl<T> crate::mpsc::bounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn bounded(capacity: usize) -> (Self::Tx, Self::Rx) {
        imp::bounded(capacity)
    }
}
