use crate::imp::crossbeam_channel as imp;

pub type Channel<T> = imp::Channel<T>;
pub type Tx<T> = imp::Tx<T>;
pub type Rx<T> = imp::Rx<T>;

impl<T> crate::mpsc::unbounded::Sender<T> for Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.send(msg)
    }
}
impl<T> crate::mpsc::unbounded::Receiver<T> for Rx<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.recv()
    }
}

impl<T> crate::mpsc::unbounded::TryReceiver<T> for Rx<T> {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.try_recv()
    }
}

impl<T> crate::mpsc::unbounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn unbounded() -> (Self::Tx, Self::Rx) {
        imp::unbounded()
    }
}
