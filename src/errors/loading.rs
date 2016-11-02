use std::io;
use std::convert::From;

/// An error during the loading state
#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  InvalidSecretText(String),
}

impl From<io::Error> for Error {

  fn from(err: io::Error) -> Error {
    Error::Io(err)
  }

}

