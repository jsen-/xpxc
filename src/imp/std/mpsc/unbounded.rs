use super::Rx;
use core::marker::PhantomData;

pub struct Tx<T>(std::sync::mpsc::Sender<T>);
impl<T> Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.0
            .send(msg)
            .map_err(|std::sync::mpsc::SendError(msg)| crate::SendError::Disconnected(msg))
    }
}
impl<T> Clone for Tx<T> {
    fn clone(&self) -> Self {
        Tx(self.0.clone())
    }
}

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

pub struct Channel<T> {
    _p: PhantomData<T>,
}
impl<T> Channel<T> {
    pub fn unbounded() -> (Tx<T>, Rx<T>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (Tx(tx), Rx(rx))
    }
}
impl<T> crate::mpsc::unbounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn unbounded() -> (Self::Tx, Self::Rx) {
        Channel::unbounded()
    }
}
