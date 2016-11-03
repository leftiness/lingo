use std::convert::Into;

use messages::{Request, Response};
use structs::State;
use traits::Messageable;

/// Initial application state
#[derive(Debug)]
pub struct Start {

  /// Path to secret toml
  secret_path: String,

}

impl Messageable for Start {

  fn tell(&self, request: Request) -> Response {
    match request {
      Request::SecretPath => Response::SecretPath(self.secret_path.to_owned()),
    }
  }

}

impl State<Start> {

  /// Initialize the application with a path to the secret toml
  ///
  /// Notice that the state machine may only be created in the initial state.
  pub fn new<T>(secret_path: T) -> Self where T: Into<String> {
    State { state: Start { secret_path: secret_path.into() } }
  }

}

