use std::io::stdin;
use std::sync::mpsc::Sender;

use termion::event::Key;
use termion::input::TermRead;

use event::{Dispatcher, Event, Listener, Publisher, Subscriber};

/// Publish stdin events
#[derive(Debug)]
pub struct Broadcaster {

  /// Transmits messages to the dispatcher
  dx: Sender<Event>,

}

impl Listener for Broadcaster {

  fn listen(&mut self) {

    let stdin = stdin();

    for key in stdin.keys() {

      match key.unwrap() {
        Key::Ctrl('c') => self.dx.send(Event::Quit).unwrap(),
        Key::Char(c) => self.dx.send(Event::KeyPress(c)).unwrap(),
        _ => continue,
      }

    }

  }

}

impl Publisher for Broadcaster {

  fn with_dispatcher(dispatcher: &Dispatcher) -> Self {
    Broadcaster { dx: dispatcher.tx().clone() }
  }

}

