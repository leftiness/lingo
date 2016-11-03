use std::convert::Into;

use messages::{Request, Response};
use structs::State;
use traits::Messageable;

/// Initial application state
#[derive(Debug)]
pub struct Start {

  /// Path to secret toml
  secret_path: Option<String>,

}

impl Start {

  fn set_secret_path<T: Into<String>>(&mut self, secret_path: T) -> Response {
    self.secret_path = Some(secret_path.into());
    Response::Thanks
  }

  fn secret_path(&self) -> Response {
    match self.secret_path {
      Some(ref path) => Response::SecretPath(path.to_owned()),
      None => Response::CantFindIt,
    }
  }

}

impl Messageable for Start {

  fn tell(&mut self, request: Request) -> Response {
    match request {
      Request::AcceptSecretPath(path) => self.set_secret_path(path),
      Request::SecretPath => self.secret_path(),
    }
  }

}

impl State<Start> {

  /// Initialize the application
  pub fn new() -> Self {
    State { state: Start { secret_path: None } }
  }

}

