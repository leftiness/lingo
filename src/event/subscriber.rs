use std::sync::mpsc::{Receiver, Sender};

use event::Event;

/// Publishers receive events
pub trait Subscriber {

  /// Reference this subscriber's transmitter
  fn tx<'a>(&'a self) -> &'a Sender<Event>;

  /// Reference this subscriber's receiver
  fn rx<'a>(&'a self) -> &'a Receiver<Event>;

  /// Act upon an event
  fn receive(&mut self, Event);

  /// Block while waiting to receive events
  fn listen(&mut self) {
    loop {
      match self.rx().recv() {
        Ok(event) => self.receive(event),
        Err(err) => panic!(err),
      }
    }
  }

}

