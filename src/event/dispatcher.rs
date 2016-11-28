use std::sync::mpsc::{self, Receiver, Sender};

use event::{Event, Subscriber};

/// Event dispatcher
#[derive(Debug)]
pub struct Dispatcher<T = Event> {

  /// Transmits messages to the receiver
  tx: Sender<T>,

  /// Receives messages from transmitters
  rx: Receiver<T>,

  /// Event subscribers
  subscribers: Vec<Sender<T>>,

}

impl Dispatcher {

  /// Register an event listener
  pub fn register<T: Subscriber<Event>>(&mut self, subscriber: &T) {
    self.subscribers.push(subscriber.tx().clone());
  }

}

impl Subscriber<Event> for Dispatcher {

  fn tx<'a>(&'a self) -> &'a Sender<Event> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Event> {
    &self.rx
  }

  fn receive(&mut self, event: Event) {

    let mut offline_subscribers: Vec<usize> = Vec::new();

    for (index, tx) in self.subscribers.iter().enumerate() {

      if let Err(_) = tx.send(event.clone()) {
        offline_subscribers.push(index);
      }

    }

    for index in offline_subscribers {
      self.subscribers.swap_remove(index);
    }

  }

}

impl Default for Dispatcher {

  fn default() -> Self {

    let (tx, rx) = mpsc::channel();

    Dispatcher {
      tx: tx,
      rx: rx,
      subscribers: Vec::new()
    }

  }

}

