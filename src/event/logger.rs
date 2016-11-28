use std::sync::mpsc::{self, Receiver, Sender};

use event::{Event, Subscriber};

/// Log events
#[derive(Debug)]
pub struct Logger {

  /// Transmits messages to the receiver
  tx: Sender<Event>,

  /// Receives messages from transmitters
  rx: Receiver<Event>,

}

impl Subscriber<Event> for Logger {

  fn tx<'a>(&'a self) -> &'a Sender<Event> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Event> {
    &self.rx
  }

  fn receive(&mut self, event: Event) {
    println!("{:?}", event);
  }

}

impl Default for Logger {

  fn default() -> Self {

    let (tx, rx) = mpsc::channel();

    Logger {
      tx: tx,
      rx: rx,
    }

  }

}

