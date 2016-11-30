use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use event::{Dispatcher, Event, Publisher, Recipient, Subscriber};
use view::Router;

/// View container
#[derive(Debug)]
pub struct Container {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

  /// Transmits messages to the dispatcher
  dx: Sender<Event>,

  /// View router
  router: Router,

}

impl Publisher for Container {

  fn with_dispatcher(dispatcher: &Dispatcher) -> Self {

    let (tx, rx) = mpsc::channel();

    Container {
      tx: tx,
      rx: rx,
      dx: dispatcher.tx().clone(),
      router: Router::default(),
    }

  }

}

impl Recipient for Container {

  fn receive(&mut self, event: Arc<Event>) -> bool {
     self.router.receive(event.as_ref())
  }

}

impl Subscriber for Container {

  fn tx<'b>(&'b self) -> &'b Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'b>(&'b self) -> &'b Receiver<Arc<Event>> {
    &self.rx
  }

}

