use std::convert::From;

use config;

/// An error stored in the application state
#[derive(Clone, Debug)]
pub enum Error {

  /// An error which occurred while loading configurations
  Config(config::Error),

}

impl From<config::Error> for Error {

  fn from(error: config::Error) -> Error {
    Error::Config(error)
  }

}

