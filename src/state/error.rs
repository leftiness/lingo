use std::convert::From;

use chat;
use config;

/// An error stored in the application state
#[derive(Clone, Debug)]
pub enum Error {

  /// An error which occurred while interacting with the Hipchat API
  Chat(chat::Error),

  /// An error which occurred while loading configurations
  Config(config::Error),

  /// The state has not successfully stored the secret toml
  MissingSecret,

}

impl From<chat::Error> for Error {

  fn from (error: chat::Error) -> Error {
    Error::Chat(error)
  }

}

impl From<config::Error> for Error {

  fn from(error: config::Error) -> Error {
    Error::Config(error)
  }

}

