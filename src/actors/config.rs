use std::io::Read;
use std::fs::File;

use errors::load::Error;
use messages::{Request, Response};
use structs;
use traits::Messageable;

use rustc_serialize::Decodable;
use toml::decode_str;

/// File name for config toml
const CONFIG_TOML: &'static str = "config.toml";

/// File name for secret toml
const SECRET_TOML: &'static str = "secret.toml";

/// Actor which handles config files
#[derive(Debug)]
pub struct Config {

  /// Loaded configurations
  pub config: Result<structs::Config, Error>,

  /// Loaded secrets
  pub secret: Result<structs::Secret, Error>,

}

impl Config {

  /// Create a new config actor
  pub fn new() -> Self {
    Config {
      config: read_toml(resource_path(CONFIG_TOML)),
      secret: read_toml(resource_path(SECRET_TOML)),
    }
  }

}

impl Messageable for Config {

  fn tell(&mut self, request: Request) -> Response {
    Response::Balderdash(request)
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

