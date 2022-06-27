use crate::imp::concurrent_queue as imp;

pub type Channel<T> = imp::Channel<T>;
pub type Tx<T> = imp::unbounded::Tx<T>;
pub type Rx<T> = imp::Rx<T>;

impl<T> crate::spmc::unbounded::Sender<T> for Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.send(msg)
    }
}
impl<T> crate::spmc::unbounded::TryReceiver<T> for Rx<T> {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.try_recv()
    }
}

impl<T> crate::spmc::unbounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn unbounded() -> (Self::Tx, Self::Rx) {
        imp::unbounded()
    }
}
