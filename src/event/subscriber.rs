use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use event::{Event, Listener, Recipient};

/// Publishers receive events
pub trait Subscriber<T = Arc<Event>>: Recipient<T> {

  /// Reference this subscriber's transmitter
  fn tx<'a>(&'a self) -> &'a Sender<T>;

  /// Reference this subscriber's receiver
  fn rx<'a>(&'a self) -> &'a Receiver<T>;

}

impl<T> Listener for Subscriber<T> {

  fn listen(&mut self) {
    loop {
      match self.rx().recv() {
        Ok(event) => self.receive(event),
        Err(err) => panic!(err),
      };
    }
  }

}

