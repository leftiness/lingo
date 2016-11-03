use std::io;
use std::convert::From;

use messages::Response;

/// An error during the loading state
#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  InvalidSecretText(String),
  UnexpectedResponse(Response),
}

impl From<io::Error> for Error {

  fn from(error: io::Error) -> Error {
    Error::Io(error)
  }

}

