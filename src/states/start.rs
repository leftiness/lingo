use std::default::Default;

use messages::{Request, Response};
use traits::Messageable;

/// Initial application state
#[derive(Debug)]
pub struct Start {}

impl Messageable for Start {

  fn tell(&mut self, request: Request) -> Response {
    Response::Balderdash(request)
  }

}

impl Default for Start {

  fn default() -> Self {
    Start {}
  }

}

