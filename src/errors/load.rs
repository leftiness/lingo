use std::io;
use std::convert::From;

/// An error during the loading state
#[derive(Debug)]
pub enum Error {

  /// Failed to load a file
  FailedToLoadFile(io::Error),

  /// The toml text was invalid
  InvalidText(String),

}

impl From<io::Error> for Error {

  fn from(error: io::Error) -> Error {
    Error::FailedToLoadFile(error)
  }

}

