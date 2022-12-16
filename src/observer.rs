type Subscribers<E> = Vec<Box<dyn Fn(&E) + Send>>;

pub struct Notifier<E> {
    subscribers: Subscribers<E>,
}

impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier {
            subscribers: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, callback: F)
    where
        F: 'static + Fn(&E) + Send,
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn notify(&self, event: E) {
        for callback in &self.subscribers {
            callback(&event);
        }
    }
}
