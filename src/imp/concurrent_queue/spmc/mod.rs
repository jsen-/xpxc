pub mod bounded;
pub mod unbounded;

use super::super::concurrent_queue as imp;

pub fn bounded<T>(capacity: usize) -> (imp::bounded::Tx<T>, imp::Rx<T>) {
    imp::bounded(capacity)
}
pub fn unbounded<T>() -> (imp::unbounded::Tx<T>, imp::Rx<T>) {
    imp::unbounded()
}
