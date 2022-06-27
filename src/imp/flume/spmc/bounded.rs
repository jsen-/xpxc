use crate::imp::flume as imp;

pub type Channel<T> = imp::Channel<T>;
pub type Tx<T> = imp::Tx<T>;
pub type Rx<T> = imp::Rx<T>;

impl<T> crate::spmc::bounded::Sender<T> for Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.send(msg)
    }
}
impl<T> crate::spmc::bounded::Receiver<T> for Rx<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.recv()
    }
}

impl<T> crate::spmc::bounded::TrySender<T> for Tx<T> {
    fn try_send(&mut self, msg: T) -> Result<(), crate::TrySendError<T>> {
        self.try_send(msg)
    }
}
impl<T> crate::spmc::bounded::TryReceiver<T> for Rx<T> {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.try_recv()
    }
}

impl<T> crate::spmc::bounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn bounded(capacity: usize) -> (Self::Tx, Self::Rx) {
        imp::bounded(capacity)
    }
}
