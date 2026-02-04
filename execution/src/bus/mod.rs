use tokio::sync::broadcast;

#[derive(Clone)]
pub struct SignalBus<T: Clone> {
    tx: broadcast::Sender<T>,
}

impl<T: Clone> SignalBus<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn publish(&self, msg: T) {
        let _ = self.tx.send(msg);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.tx.subscribe()
    }
}
