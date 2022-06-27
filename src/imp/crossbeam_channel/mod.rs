pub mod mpsc;
pub mod spmc;

use core::marker::PhantomData;

pub struct Tx<T>(crossbeam_channel::Sender<T>);
impl<T> Tx<T> {
    pub fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.0
            .send(msg)
            .map_err(|crossbeam_channel::SendError(msg)| crate::SendError::Disconnected(msg))
    }
    pub fn try_send(&mut self, msg: T) -> Result<(), crate::TrySendError<T>> {
        self.0
            .try_send(msg)
            .map_err(|err| match err {
                crossbeam_channel::TrySendError::Disconnected(msg) => crate::TrySendError::Disconnected(msg),
                crossbeam_channel::TrySendError::Full(msg) => crate::TrySendError::Full(msg),
            })
    }
}
impl<T> Clone for Tx<T> {
    fn clone(&self) -> Self {
        Tx(self.0.clone())
    }
}

pub struct Rx<T>(crossbeam_channel::Receiver<T>);
impl<T> Rx<T> {
    pub fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.0.recv().map_err(|err| match err {
            crossbeam_channel::RecvError => crate::RecvError::Disconnected,
        })
    }
    pub fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.0.try_recv().map_err(|err| match err {
            crossbeam_channel::TryRecvError::Disconnected => crate::TryRecvError::Disconnected,
            crossbeam_channel::TryRecvError::Empty => crate::TryRecvError::Empty,
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

fn bounded<T>(capacity: usize) -> (Tx<T>, Rx<T>) {
    let (tx, rx) = crossbeam_channel::bounded(capacity);
    (Tx(tx), Rx(rx))
}
fn unbounded<T>() -> (Tx<T>, Rx<T>) {
    let (tx, rx) = crossbeam_channel::unbounded();
    (Tx(tx), Rx(rx))
}
