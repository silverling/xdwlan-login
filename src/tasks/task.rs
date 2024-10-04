use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
pub enum AppEvent {
    Quit,
}

pub trait Task {
    /// Every task communicates with a daemon or manager through a pair of channels. It uses `notifier` to send events to the daemon or manager, and uses `receiver` to receive events from the daemon or manager.
    ///
    /// For now, the only event is `Quit`, which means the daemon or manager should quit.
    fn run(&self, notifier: Sender<AppEvent>, receiver: Receiver<AppEvent>) -> anyhow::Result<()>;
}
