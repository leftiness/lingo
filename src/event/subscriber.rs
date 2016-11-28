use std::sync::mpsc::{Receiver, Sender};

/// Publishers receive events
pub trait Subscriber<T> {

  /// Reference this subscriber's transmitter
  fn tx<'a>(&'a self) -> &'a Sender<T>;

  /// Reference this subscriber's receiver
  fn rx<'a>(&'a self) -> &'a Receiver<T>;

  /// Act upon an event
  fn receive(&mut self, T);

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

