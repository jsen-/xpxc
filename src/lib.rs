pub mod mpsc;
pub mod spmc;

pub mod imp;

#[derive(Debug)]
pub enum SendError<T> {
    Disconnected(T),
}
#[derive(Debug)]
pub enum RecvError {
    Disconnected,
}

#[derive(Debug)]
pub enum TrySendError<T> {
    Disconnected(T),
    Full(T),
}
#[derive(Debug)]
pub enum TryRecvError {
    Disconnected,
    Empty,
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;

    fn mpsc_bounded_send_recv<X, T: mpsc::bounded::Sender<X>, R: mpsc::bounded::Receiver<X>>(_: &(T, R)) {}
    fn mpsc_bounded_try_send_recv<X, T: mpsc::bounded::TrySender<X>, R: mpsc::bounded::TryReceiver<X>>(_: &(T, R)) {}

    fn mpsc_unbounded_send_recv<X, T: mpsc::unbounded::Sender<X>, R: mpsc::unbounded::Receiver<X>>(_: &(T, R)) {}
    fn mpsc_unbounded_try_send_recv<X, T, R: mpsc::unbounded::TryReceiver<X>>(_: &(T, R)) {}

    fn spmc_bounded_send_recv<X, T: spmc::bounded::Sender<X>, R: mpsc::bounded::Receiver<X>>(_: &(T, R)) {}
    fn spmc_bounded_try_send_recv<X, T: spmc::bounded::TrySender<X>, R: mpsc::bounded::TryReceiver<X>>(_: &(T, R)) {}

    fn spmc_unbounded_send_recv<X, T: spmc::unbounded::Sender<X>, R: spmc::unbounded::Receiver<X>>(_: &(T, R)) {}
    fn spmc_unbounded_try_send_recv<X, T, R: spmc::unbounded::TryReceiver<X>>(_: &(T, R)) {}

    #[test]
    #[cfg(feature = "crossbeam-channel")]
    fn trait_coverage_crossbeam_channel() {
        use imp::crossbeam_channel as imp;
        let c = imp::mpsc::bounded::<u32>(10);
        mpsc_bounded_send_recv(&c);
        mpsc_bounded_try_send_recv(&c);

        let c = imp::mpsc::unbounded::<u32>();
        mpsc_unbounded_send_recv(&c);
        mpsc_unbounded_try_send_recv(&c);

        let c = imp::spmc::bounded::<u32>(10);
        spmc_bounded_send_recv(&c);
        spmc_bounded_try_send_recv(&c);

        let c = imp::spmc::unbounded::<u32>();
        spmc_unbounded_send_recv(&c);
        spmc_unbounded_try_send_recv(&c);
    }

    #[test]
    #[cfg(feature = "flume")]
    fn trait_coverage_flume() {
        use imp::flume as imp;
        let c = imp::mpsc::bounded::<u32>(10);
        mpsc_bounded_send_recv(&c);
        mpsc_bounded_try_send_recv(&c);

        let c = imp::mpsc::unbounded::<u32>();
        mpsc_unbounded_send_recv(&c);
        mpsc_unbounded_try_send_recv(&c);

        let c = imp::spmc::bounded::<u32>(10);
        spmc_bounded_send_recv(&c);
        spmc_bounded_try_send_recv(&c);

        let c = imp::spmc::unbounded::<u32>();
        spmc_unbounded_send_recv(&c);
        spmc_unbounded_try_send_recv(&c);
    }

    #[test]
    #[cfg(feature = "std")]
    fn trait_coverage_std() {
        use imp::std as imp;
        let c = imp::mpsc::bounded::<u32>(10);
        mpsc_bounded_send_recv(&c);
        mpsc_bounded_try_send_recv(&c);

        let c = imp::mpsc::unbounded::<u32>();
        mpsc_unbounded_send_recv(&c);
        mpsc_unbounded_try_send_recv(&c);
    }

    #[test]
    #[cfg(feature = "spmc")]
    fn trait_coverage_spmc() {
        use imp::spmc as imp;
        let c = imp::unbounded::<u32>();
        spmc_unbounded_send_recv(&c);
    }

    #[test]
    #[cfg(feature = "concurrent-queue")]
    fn trait_coverage_concurrent_queue() {
        use imp::concurrent_queue as imp;
        let c = imp::mpsc::bounded::<u32>(10);
        mpsc_bounded_try_send_recv(&c);

        let c = imp::mpsc::unbounded::<u32>();
        mpsc_unbounded_try_send_recv(&c);

        let c = imp::spmc::bounded::<u32>(10);
        spmc_bounded_try_send_recv(&c);

        let c = imp::spmc::unbounded::<u32>();
        spmc_unbounded_try_send_recv(&c);
    }
}
