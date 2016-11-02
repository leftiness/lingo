use std::convert::Into;

use structs::State;

/// Initial application state
#[derive(Debug)]
pub struct Initialize {

  /// Path to secret toml
  pub secret_path: String,

}

impl State<Initialize> {

  /// Initialize the application with a path to the secret toml
  ///
  /// Notice that the state machine may only be created in the initial state.
  pub fn new<T>(secret_path: T) -> Self where T: Into<String> {
    State { state: Initialize { secret_path: secret_path.into() } }
  }

}

