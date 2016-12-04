use std::fmt::{self, Debug};
use std::io::{stdout, Stdout, Write};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use termion::raw::{RawTerminal, IntoRawMode};

use event::{Dispatcher, Event, Publisher, Recipient, Subscriber};
use view::{Clear, Component, View};

/// View container
pub struct Container {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

  /// Transmits messages to the dispatcher
  dx: Sender<Event>,

  /// Stdout used to render the application
  stdout: RawTerminal<Stdout>,

}

impl Publisher for Container {

  fn with_dispatcher(dispatcher: &Dispatcher) -> Self {

    let (tx, rx) = mpsc::channel();

    Container {
      tx: tx,
      rx: rx,
      dx: dispatcher.tx().clone(),
      stdout: stdout().into_raw_mode().unwrap(),
    }

  }

}

impl Recipient for Container {

  fn receive(&mut self, event: Arc<Event>) -> bool {

    let mut is_relevant = true;

    let stdout = &mut self.stdout.lock();

    match *event {
      Event::StateUpdate(ref state) => View::render(stdout, state).unwrap(),
      Event::Quit => Clear::render(stdout, &()).unwrap(),
      _ => is_relevant = false,
    }

    if is_relevant {
      stdout.flush().unwrap();
    }

    is_relevant

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

impl Debug for Container {

  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(
      formatter,
      "Container {{ tx: {:?}, rx: {:?}, dx: {:?}, stdout: RawTerminal }}",
      self.tx,
      self.rx,
      self.dx,
    )
  }

}

