use std::sync::mpsc::{Receiver, Sender, SendError};

use event::Event;

/// A broadcaster has methods to send and receive events
pub trait Broadcaster {

  /// Create a new broadcaster with a dispatcher
  fn with_dispatcher(Sender<Event>) -> Self;

  /// Reference this broadcaster's transmitter
  fn tx<'a>(&'a self) -> &'a Sender<Event>;

  /// Reference this broadcaster's receiver
  fn rx<'a>(&'a self) -> &'a Receiver<Event>;

  /// Reference this broadcaster's dispatcher
  fn dispatcher<'a>(&'a self) -> &'a Sender<Event>;

  /// Act upon an event
  fn receive(&mut self, Event);

  /// Send a message to the dispatcher
  fn dispatch(&self, event: Event) -> Result<(), SendError<Event>> {
    self.dispatcher().send(event)
  }

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

