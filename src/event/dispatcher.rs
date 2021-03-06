use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use event::{Event, Recipient, Subscriber};

/// Event dispatcher
#[derive(Debug)]
pub struct Dispatcher<T = Event> {

  /// Transmits messages to the receiver
  tx: Sender<T>,

  /// Receives messages from transmitters
  rx: Receiver<T>,

  /// Event subscribers
  subscribers: Vec<Sender<Arc<T>>>,

}

impl Dispatcher {

  /// Register an event listener
  pub fn register<T: Subscriber>(&mut self, subscriber: &T) {
    self.subscribers.push(subscriber.tx().clone());
  }

}

impl Recipient<Event> for Dispatcher {

  fn receive(&mut self, event: Event) -> bool {

    let mut offline_subscribers: Vec<usize> = Vec::new();

    let always_relevant = true;
    let arc = Arc::new(event);

    for (index, tx) in self.subscribers.iter().enumerate() {

      if let Err(_) = tx.send(arc.clone()) {
        offline_subscribers.push(index);
      }

    }

    for index in offline_subscribers {
      self.subscribers.swap_remove(index);
    }

    always_relevant

  }

}

impl Subscriber<Event> for Dispatcher {

  fn tx<'a>(&'a self) -> &'a Sender<Event> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Event> {
    &self.rx
  }

}

impl Default for Dispatcher {

  fn default() -> Self {

    let (tx, rx) = mpsc::channel();

    Dispatcher {
      tx: tx,
      rx: rx,
      subscribers: Vec::new(),
    }

  }

}

