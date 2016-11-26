use std::sync::mpsc::{self, Receiver, Sender};

use event::{Broadcaster, Event};

/// Event dispatcher
#[derive(Debug)]
pub struct Dispatcher {

  /// Transmits messages to the receiver
  tx: Sender<Event>,

  /// Receives messages from transmitters
  rx: Receiver<Event>,

  /// Transmits messages to the dispatcher
  dispatcher: Sender<Event>,

  /// Event subscribers
  subscribers: Vec<Sender<Event>>,

}

impl Dispatcher {

  /// Create a new dispatcher
  pub fn new() -> Self {

    let (tx, rx) = mpsc::channel::<Event>();

    Dispatcher {
      tx: tx.clone(),
      rx: rx,
      dispatcher: tx,
      subscribers: Vec::new()
    }

  }

  /// Register an event listener
  pub fn register(&mut self, tx: Sender<Event>) -> usize {
    self.subscribers.push(tx);
    self.subscribers.len()
  }

  /// Deregister an event listener by index
  pub fn deregister(&mut self, index: usize) {
    self.subscribers.swap_remove(index);
  }

}

impl Broadcaster for Dispatcher {

  fn with_dispatcher(dispatcher: Sender<Event>) -> Self {

    let (tx, rx) = mpsc::channel::<Event>();

    Dispatcher {
      tx: tx,
      rx: rx,
      dispatcher: dispatcher,
      subscribers: Vec::new()
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

