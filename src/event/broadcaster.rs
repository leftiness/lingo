use std::sync::mpsc::{Receiver, Sender};

use event::Event;

/// A broadcaster has methods to send and receive events
pub trait Broadcaster {

  /// Create a new broadcaster with a dispatcher
  fn with_dispatcher<T: Broadcaster>(&T) -> Self;

  /// Reference this broadcaster's transmitter
  fn tx<'a>(&'a self) -> &'a Sender<Event>;

  /// Reference this broadcaster's receiver
  fn rx<'a>(&'a self) -> &'a Receiver<Event>;

  /// Reference this broadcaster's dispatcher
  fn dx<'a>(&'a self) -> &'a Sender<Event>;

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

