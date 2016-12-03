use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use event::{Dispatcher, Event, Publisher, Recipient, Subscriber};
use state::{Error, State};

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

impl Container {

  /// React to a request to load rooms
  fn load_rooms(&self) {

    let event = match self.state.secret {
      Some(ref secret) => Event::LoadRoomsWithSecret(secret.clone()),
      None => Event::StateErr(Error::MissingSecret),
    };

    self.dx.send(event).unwrap();

  }

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

    let relevant_to_state = self.state.receive(event.as_ref());

    if relevant_to_state {
      self.dx.send(Event::StateUpdate(self.state.clone())).unwrap();
    }

    let mut relevant_to_container = true;

    match *event {
      Event::LoadRooms => self.load_rooms(),
      _ => relevant_to_container = false,
    }

    relevant_to_state || relevant_to_container

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

