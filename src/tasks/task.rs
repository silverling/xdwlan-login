use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
pub enum AppEvent {
    Quit,
}

pub trait Task {
    fn run(&self, sender: Sender<AppEvent>, receiver: Receiver<AppEvent>) -> anyhow::Result<()>;
}
