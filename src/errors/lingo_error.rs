use std::io;
use std::convert::From;

#[derive(Debug)]
pub enum LingoError {
  Io(io::Error),
  InvalidSecretText(String),
}

impl From<io::Error> for LingoError {

  fn from(err: io::Error) -> LingoError {
    LingoError::Io(err)
  }

}

