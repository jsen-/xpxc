pub trait Sender<T> {
    fn send(&mut self, t: T) -> Result<(), crate::SendError<T>>;
}
pub trait Receiver<T>: Clone {
    fn recv(&mut self) -> Result<T, crate::RecvError>;
}

pub trait TrySender<T> {
    fn try_send(&mut self, t: T) -> Result<(), crate::TrySendError<T>>;
}
pub trait TryReceiver<T>: Clone {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError>;
}

pub trait Channel<T> {
    type Tx;
    type Rx;
    fn bounded(capacity: usize) -> (Self::Tx, Self::Rx);
}
