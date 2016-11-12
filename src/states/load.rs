use std::convert::From;
use std::io::Read;
use std::fs::File;

use toml::decode_str;

use errors::load::Error;
use messages::{Request, Response};
use states::Start;
use structs::Secret;
use traits::Messageable;

/// State which loads configurations
#[derive(Debug)]
pub struct Load {

  /// Loaded configurations
  pub secret: Result<Secret, Error>,

}

impl Load {

  /// Load and decode the secret toml from the provided path
  fn load_secret(secret_path: &String) -> Result<Secret, Error> {

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

  fn tell(&mut self, request: Request) -> Response {
    match request {
      _ => Response::Balderdash(request),
    }
  }

}

impl From<Start> for Load {

  fn from(start: Start) -> Load {

    let secret = match start.secret_path {
      Some(path) => Load::load_secret(&path),
      None => Err(Error::MissingSecretPath),
    };

    Load { secret: secret }

  }

}

