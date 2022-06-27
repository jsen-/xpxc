use core::marker::PhantomData;

pub struct Tx<T: Send>(::spmc::Sender<T>);
impl<T: Send> Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.0
            .send(msg)
            .map_err(|::spmc::SendError(msg)| crate::SendError::Disconnected(msg))
    }
}
impl<T: Send> crate::spmc::unbounded::Sender<T> for Tx<T> {
    fn send(&mut self, msg: T) -> Result<(), crate::SendError<T>> {
        self.send(msg)
    }
}

pub struct Rx<T: Send>(::spmc::Receiver<T>);
impl<T: Send> Rx<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.0
            .recv()
            .map_err(|::spmc::RecvError| crate::RecvError::Disconnected)
    }
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.0.try_recv().map_err(|err| match err {
            spmc::TryRecvError::Disconnected => crate::TryRecvError::Disconnected,
            spmc::TryRecvError::Empty => crate::TryRecvError::Empty,
        })
    }
}
impl<T: Send> Clone for Rx<T> {
    fn clone(&self) -> Self {
        Rx(self.0.clone())
    }
}
impl<T: Send> crate::spmc::unbounded::Receiver<T> for Rx<T> {
    fn recv(&mut self) -> Result<T, crate::RecvError> {
        self.recv()
    }
}

impl<T: Send> crate::spmc::unbounded::TryReceiver<T> for Rx<T> {
    fn try_recv(&mut self) -> Result<T, crate::TryRecvError> {
        self.try_recv()
    }
}

pub struct Channel<T: Send> {
    _p: PhantomData<T>,
}
impl<T: Send> Channel<T> {
    pub fn unbounded() -> (Tx<T>, Rx<T>) {
        let (tx, rx) = ::spmc::channel();
        (Tx(tx), Rx(rx))
    }
}
impl<T: Send> crate::spmc::unbounded::Channel<T> for Channel<T> {
    type Tx = Tx<T>;
    type Rx = Rx<T>;
    fn unbounded() -> (Self::Tx, Self::Rx) {
        Channel::unbounded()
    }
}
