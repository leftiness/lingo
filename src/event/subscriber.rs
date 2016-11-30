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

impl<T> Listener for Subscriber<T> where T: AsRef<Event> {

  fn listen(&mut self) {

    let mut should_break = false;

    loop {

      let event = match self.rx().recv() {
        Ok(event) => event,
        Err(err) => panic!(err),
      };

      if let Event::Quit = *event.as_ref() {
        should_break = true;
      }

      self.receive(event);

      if should_break {
        break;
      }

    }

  }

}

