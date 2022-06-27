pub mod bounded;
pub mod unbounded;

use super::super::flume as imp;

pub fn bounded<T>(capacity: usize) -> (imp::Tx<T>, imp::Rx<T>) {
    imp::bounded(capacity)
}
pub fn unbounded<T>() -> (imp::Tx<T>, imp::Rx<T>) {
    imp::unbounded()
}
