use toml::decode_str;

use std::convert::From;
use std::io::Read;
use std::fs::File;

use errors::loading::Error;
use messages::{Request, Response};
use states::Start;
use structs::{Secret, State};
use traits::Messageable;

/// State which loads configurations
#[derive(Debug)]
pub struct Load {

  /// Loaded configurations
  secret: Result<Secret, Error>,

}

impl Load {

  /// Load and decode the secret toml from the provided path
  pub fn load_secret(secret_path: &String) -> Result<Secret, Error> {

    let mut secret_file = try!(File::open(secret_path));
    let mut secret_text = String::new();

    try!(secret_file.read_to_string(&mut secret_text));

    match decode_str(secret_text.as_str()) {
      Some(decoded) => Ok(decoded),
      None => Err(Error::InvalidSecretText(secret_text)),
    }

  }

}

impl Messageable for Load {

  fn tell(&self, request: Request) -> Response {
    match request {
      _ => Response::Balderdash(request),
    }
  }

}

impl From<State<Start>> for State<Load> {

  fn from(start: State<Start>) -> State<Load> {

    let secret = match start.tell(Request::SecretPath) {
      Response::SecretPath(path) => Load::load_secret(&path),
      response @ _ => Err(Error::UnexpectedResponse(response)),
    };

    State { state: Load { secret: secret } }

  }

}

