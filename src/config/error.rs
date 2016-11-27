use std::error::Error as ErrorTrait;
use std::io;
use std::convert::From;

/// An error during the loading state
#[derive(Clone, Debug)]
pub enum Error {

  /// Failed to load a file
  FailedToLoadFile(String),

  /// The toml text was invalid
  InvalidText(String),

}

impl From<io::Error> for Error {

  fn from(error: io::Error) -> Error {
    Error::FailedToLoadFile(error.description().to_owned())
  }

}

