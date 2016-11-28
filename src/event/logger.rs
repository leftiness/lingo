use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use event::{Event, Subscriber};

/// Log events
#[derive(Debug)]
pub struct Logger {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

}

impl Subscriber<Arc<Event>> for Logger {

  fn tx<'a>(&'a self) -> &'a Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Arc<Event>> {
    &self.rx
  }

  fn receive(&mut self, event: Arc<Event>) {
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

