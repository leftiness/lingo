use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use event::{Dispatcher, Event, Publisher, Subscriber};
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

impl Subscriber for Container {

  fn tx<'a>(&'a self) -> &'a Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Arc<Event>> {
    &self.rx
  }

  fn receive(&mut self, event: Arc<Event>) {
    if self.state.update(event.as_ref()) {
      self.dx.send(Event::StateUpdate(self.state.clone())).unwrap();
    }
  }

}

