pub trait Sender<T>: Clone {
    fn send(&mut self, t: T) -> Result<(), crate::SendError<T>>;
}
pub trait Receiver<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError>;
}

pub trait TryReceiver<T> {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError>;
}

pub trait Channel<T> {
    type Tx;
    type Rx;
    fn unbounded() -> (Self::Tx, Self::Rx);
}
