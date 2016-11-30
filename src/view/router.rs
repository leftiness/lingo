use std::fmt::{self, Debug};
use std::io::{self, stdout, Stdout, Write};

use termion::raw::{RawTerminal, IntoRawMode};

use event::{Event, Recipient};
use state::State;
use view::{Clear, Component};

/// View router
pub struct Router {

  /// Stdout used to render the application
  stdout: RawTerminal<Stdout>,

}

impl Component for Router {

  fn render<T: Write>(stdout: &mut T, state: &State) -> io::Result<()> {
    match state.view.as_str() {
      "/" => write!(stdout, "{:?}", state),
      _ => unimplemented!(),
    }
  }

}

impl<'a> Recipient<&'a Event> for Router {

  fn receive(&mut self, event: &Event) -> bool {

    let mut is_relevant = true;

    let stdout = &mut self.stdout.lock();

    match *event {
      Event::StateUpdate(ref state) => Router::render(stdout, state).unwrap(),
      Event::Quit => Clear::render(stdout, &()).unwrap(),
      _ => is_relevant = false,
    }

    if is_relevant {
      stdout.flush().unwrap();
    }

    is_relevant

  }

}

impl Default for Router {

  fn default() -> Self {
    Router {
      stdout: stdout().into_raw_mode().unwrap(),
    }
  }

}

impl Debug for Router {

  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Router {{}}")
  }

}

