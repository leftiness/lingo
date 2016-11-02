use toml::decode_str;

use std::convert::From;
use std::io::Read;
use std::fs::File;

use errors::loading::Error;
use structs::{Secret, State};
use structs::states::Start;

/// State which loads configurations
#[derive(Debug)]
pub struct Load {

  /// Loaded configurations
  pub secret: Result<Secret, Error>,

}

impl Load {

  /// Load and decode the secret toml from the provided path
  pub fn load_secret(secret_path: String) -> Result<Secret, Error> {

    let mut secret_file = try!(File::open(secret_path));
    let mut secret_text = String::new();

    try!(secret_file.read_to_string(&mut secret_text));

    match decode_str(secret_text.as_str()) {
      Some(decoded) => Ok(decoded),
      None => Err(Error::InvalidSecretText(secret_text)),
    }

  }

}

impl From<State<Start>> for State<Load> {

  fn from(start: State<Start>) -> State<Load> {

    let secret = Load::load_secret(start.state.secret_path);

    State { state: Load { secret: secret } }

  }

}

