use crossbeam::channel::Receiver;

pub(crate) trait Subscriber<T> {
    fn subscribe(&mut self, rx: Receiver<T>);
}
