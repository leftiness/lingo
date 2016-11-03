use messages::{Request, Response};
use traits::Messageable;

/// Application state machine
#[derive(Debug)]
pub struct State<S: Messageable> {

  /// Arbitrary application state
  pub state: S,

}

impl<S: Messageable> Messageable for State<S> {

  fn tell(&mut self, request: Request) -> Response {
    self.state.tell(request)
  }

}

