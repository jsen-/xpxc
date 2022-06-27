use super::Rx;
use core::marker::PhantomData;

pub struct Tx<T>(std::sync::mpsc::SyncSender<T>);
impl<T> Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.0
            .send(msg)
            .map_err(|std::sync::mpsc::SendError(msg)| crate::SendError::Disconnected(msg))
    }
    fn try_send(&mut self, msg: T) -> Result<(), crate::TrySendError<T>> {
        self.0
            .try_send(msg)
            .map_err(|err| match err {
                std::sync::mpsc::TrySendError::Disconnected(msg) => crate::TrySendError::Disconnected(msg),
                std::sync::mpsc::TrySendError::Full(msg) => crate::TrySendError::Full(msg),
            })
    }
}
impl<T> Clone for Tx<T> {
    fn clone(&self) -> Self {
        Tx(self.0.clone())
    }
}

impl<T> crate::mpsc::bounded::Sender<T> for Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.send(msg)
    }
}
impl<T> crate::mpsc::bounded::Receiver<T> for Rx<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.recv()
    }
}

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

pub struct Channel<T> {
    _p: PhantomData<T>,
}
impl<T> Channel<T> {
    pub fn bounded(capacity: usize) -> (Tx<T>, Rx<T>) {
        let (tx, rx) = std::sync::mpsc::sync_channel(capacity);
        (Tx(tx), Rx(rx))
    }
}
impl<T> crate::mpsc::bounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn bounded(capacity: usize) -> (Self::Tx, Self::Rx) {
        Channel::bounded(capacity)
    }
}
