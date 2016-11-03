use std::io;
use std::convert::From;

use messages::Response;

/// An error during the loading state
#[derive(Debug)]
pub enum Error {

  /// Failed to load a file
  FailedToLoadFile(io::Error),

  /// A path to the secret toml was not provided
  MissingSecretPath,

  /// The secret toml text was invalid
  InvalidSecretText(String),

  /// Received an unexpected response to a request
  UnexpectedResponse(Response),

}

impl From<io::Error> for Error {

  fn from(error: io::Error) -> Error {
    Error::FailedToLoadFile(error)
  }

}

