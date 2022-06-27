pub mod bounded;
pub mod unbounded;

pub struct Rx<T>(std::sync::mpsc::Receiver<T>);
impl<T> Rx<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.0
            .recv()
            .map_err(|std::sync::mpsc::RecvError| crate::RecvError::Disconnected)
    }
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.0.try_recv().map_err(|err| match err {
            std::sync::mpsc::TryRecvError::Disconnected => crate::TryRecvError::Disconnected,
            std::sync::mpsc::TryRecvError::Empty => crate::TryRecvError::Empty,
        })
    }
}

pub fn bounded<T>(capacity: usize) -> (bounded::Tx<T>, Rx<T>) {
    bounded::Channel::bounded(capacity)
}
pub fn unbounded<T>() -> (unbounded::Tx<T>, Rx<T>) {
    unbounded::Channel::unbounded()
}
