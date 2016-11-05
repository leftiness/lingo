use std::convert::From;

use messages::{Request, Response};
use states::Start;
use traits::{Messageable, Transition};

/// Application state machine
#[derive(Debug)]
pub struct State<S: Messageable> {

  /// Arbitrary application state
  state: S,

}

impl<S: Messageable> Messageable for State<S> {

  fn tell(&mut self, request: Request) -> Response {
    self.state.tell(request)
  }

}

impl State<Start> {

  /// Initialize the application
  pub fn new() -> Self {
    State { state: Start::default() }
  }

}

impl<T, U> Transition<State<T>> for State<U>
  where T: Messageable, U: Messageable, U: From<T> {

  fn from(t: State<T>) -> State<U> {
    State { state: U::from(t.state) }
  }

}

