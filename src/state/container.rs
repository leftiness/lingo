use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use event::{Dispatcher, Event, Publisher, Recipient, Subscriber};
use state::State;

/// Application state container
#[derive(Debug)]
pub struct Container {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

  /// Transmits messages to the dispatcher
  dx: Sender<Event>,

  /// Application state
  state: State,

}

impl Publisher for Container {

  fn with_dispatcher(dispatcher: &Dispatcher) -> Self {

    let (tx, rx) = mpsc::channel();

    Container {
      tx: tx,
      rx: rx,
      dx: dispatcher.tx().clone(),
      state: State::default(),
    }

  }

}

impl Recipient for Container {

  fn receive(&mut self, event: Arc<Event>) -> bool {

    let is_relevant = self.state.receive(event.as_ref());

    if is_relevant {
      self.dx.send(Event::StateUpdate(self.state.clone())).unwrap();
    }

    is_relevant

  }

}

impl Subscriber for Container {

  fn tx<'a>(&'a self) -> &'a Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Arc<Event>> {
    &self.rx
  }

}

