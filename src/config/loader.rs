use std::io::Read;
use std::fs::File;
use std::sync::mpsc::{self, Receiver, Sender};

use rustc_serialize::Decodable;
use toml::decode_str;

use config::{Error, Preference, Secret};
use event::{Broadcaster, Event};

/// File name for config toml
const PREFERENCE_TOML: &'static str = "preference.toml";

/// File name for secret toml
const SECRET_TOML: &'static str = "secret.toml";

/// Loads configuration files
#[derive(Debug)]
pub struct Loader {

  /// Transmits messages to the receiver
  tx: Sender<Event>,

  /// Receives messages from transmitters
  rx: Receiver<Event>,

  /// Transmits messages to the dispatcher
  dispatcher: Sender<Event>,

}

impl Loader {

  fn load_preference(&self) {

    let event = match read_toml::<Preference>(resource_path(PREFERENCE_TOML)) {
      Ok(preference) => Event::LoadPreferenceOk(preference),
      Err(err) => Event::LoadPreferenceErr(err),
    };

    self.dispatch(event).unwrap();

  }

  fn load_secret(&self) {

    let event = match read_toml::<Secret>(resource_path(SECRET_TOML)) {
      Ok(secret) => Event::LoadSecretOk(secret),
      Err(err) => Event::LoadSecretErr(err),
    };

    self.dispatch(event).unwrap();

  }

}

impl Broadcaster for Loader {

  /// Create a new loader
  fn with_dispatcher(dispatcher: Sender<Event>) -> Self {

    let (tx, rx) = mpsc::channel::<Event>();

    Loader {
      tx: tx,
      rx: rx,
      dispatcher: dispatcher,
    }

  }

  fn tx<'a>(&'a self) -> &'a Sender<Event> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Event> {
    &self.rx
  }

  fn dispatcher<'a>(&'a self) -> &'a Sender<Event> {
    &self.dispatcher
  }

  fn receive(&mut self, event: Event) {
    match event {
      Event::LoadPreference => self.load_preference(),
      Event::LoadSecret => self.load_secret(),
      _ => (),
    }
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

