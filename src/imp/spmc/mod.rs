pub mod unbounded;

pub fn unbounded<T: Send>() -> (unbounded::Tx<T>, unbounded::Rx<T>) {
    unbounded::Channel::unbounded()
}
