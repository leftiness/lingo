use messages::{Request, Response};
use traits::Messageable;

/// Application state machine
pub struct State<S: Messageable> {

  /// Arbitrary application state
  pub state: S,

}

impl<S: Messageable> Messageable for State<S> {

  fn tell(&self, request: Request) -> Response {
    self.state.tell(request)
  }

}

