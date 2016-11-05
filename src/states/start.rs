use std::convert::Into;
use std::default::Default;

use messages::{Request, Response};
use traits::Messageable;

/// Initial application state
#[derive(Debug)]
pub struct Start {

  /// Path to secret toml
  pub secret_path: Option<String>,

}

impl Start {

  fn set_secret_path<T: Into<String>>(&mut self, secret_path: T) -> Response {
    self.secret_path = Some(secret_path.into());
    Response::Habadagus
  }

}

impl Messageable for Start {

  fn tell(&mut self, request: Request) -> Response {
    match request {
      Request::AcceptSecretPath(path) => self.set_secret_path(path),
    }
  }

}

impl Default for Start {

  fn default() -> Start {
    Start { secret_path: None }
  }

}

