use std::convert::{From, Into};
use std::io::Read;
use std::fs::File;

use rustc_serialize::Decodable;
use toml::decode_str;

use errors::load::Error;
use messages::{Request, Response};
use states::Start;
use structs::{Config, Secret};
use traits::Messageable;

/// File name for config toml
const CONFIG_TOML: &'static str = "config.toml";

/// File name for secret toml
const SECRET_TOML: &'static str = "secret.toml";

/// State which loads configurations
#[derive(Debug)]
pub struct Load {

  /// Loaded configurations
  pub config: Result<Config, Error>,

  /// Loaded secrets
  pub secret: Result<Secret, Error>,

}

impl Messageable for Load {

  fn tell(&mut self, request: Request) -> Response {
    Response::Balderdash(request)
  }

}

impl From<Start> for Load {

  fn from(_: Start) -> Load {

    let config_path = resource_path(CONFIG_TOML);
    let secret_path = resource_path(SECRET_TOML);

    let config = read_toml::<Config>(config_path);
    let secret = read_toml::<Secret>(secret_path);

    Load { config: config, secret: secret }

  }

}

/// Return the place where a configuration should go
fn resource_path<T: Into<String>>(resource_name: T) -> String {

  let config_home: String = match option_env!("XDG_CONFIG_HOME") {
    Some(path) => path.to_owned(),
    None => format!("{}/.config", env!("HOME")),
  };

  let config_path: String = format!(
    "{}/{}/{}",
    config_home,
    env!("CARGO_PKG_NAME"),
    resource_name.into()
  );

  config_path

}

/// Load and decode a toml from the provided path
fn read_toml<T>(path: String) -> Result<T, Error> where T: Decodable {

  let mut file = try!(File::open(&path));
  let mut text = String::new();

  try!(file.read_to_string(&mut text));

  match decode_str(text.as_str()) {
    Some(decoded) => Ok(decoded),
    None => Err(Error::InvalidText(text)),
  }

}

