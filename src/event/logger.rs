use std::sync::mpsc::{self, Receiver, Sender};

use event::{Broadcaster, Event};

/// Log events
#[derive(Debug)]
pub struct Logger {

  /// Transmits messages to the receiver
  tx: Sender<Event>,

  /// Receives messages from transmitters
  rx: Receiver<Event>,

  /// Transmits messages to the dispatcher
  dispatcher: Sender<Event>,

}

impl Broadcaster for Logger {

  /// Create a new loader
  fn with_dispatcher<T: Broadcaster>(dispatcher: &T) -> Self {

    let (tx, rx) = mpsc::channel::<Event>();

    Logger {
      tx: tx,
      rx: rx,
      dispatcher: dispatcher.tx().clone(),
    }

  }

  fn tx<'a>(&'a self) -> &'a Sender<Event> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Event> {
    &self.rx
  }

  fn dispatcher<'a>(&'a self) -> &'a Sender<Event> {
    &self.dispatcher
  }

  fn receive(&mut self, event: Event) {
    println!("{:?}", event);
  }

}

