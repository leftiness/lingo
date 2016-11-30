use std::io::Read;
use std::fs::File;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use rustc_serialize::Decodable;
use toml::decode_str;

use config::{Error, Preference, Secret};
use event::{Dispatcher, Event, Publisher, Recipient, Subscriber};
use xdg::Xdg;

/// File name for config toml
const PREFERENCE_TOML: &'static str = "preference.toml";

/// File name for secret toml
const SECRET_TOML: &'static str = "secret.toml";

/// Loads configuration files
#[derive(Debug)]
pub struct Loader {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

  /// Transmits messages to the dispatcher
  dx: Sender<Event>,

}

impl Loader {

  fn load_preference(&self) {

    let path = Xdg::Config.resource(PREFERENCE_TOML);

    let event = match read_toml::<Preference>(path) {
      Ok(preference) => Event::LoadPreferenceOk(preference),
      Err(err) => Event::LoadPreferenceErr(err),
    };

    self.dx.send(event).unwrap();

  }

  fn load_secret(&self) {

    let path = Xdg::Config.resource(SECRET_TOML);

    let event = match read_toml::<Secret>(path) {
      Ok(secret) => Event::LoadSecretOk(secret),
      Err(err) => Event::LoadSecretErr(err),
    };

    self.dx.send(event).unwrap();

  }

}

impl Publisher for Loader {

  fn with_dispatcher(dispatcher: &Dispatcher) -> Self {

    let (tx, rx) = mpsc::channel();

    Loader {
      tx: tx,
      rx: rx,
      dx: dispatcher.tx().clone(),
    }

  }

}

impl Recipient for Loader {

  fn receive(&mut self, event: Arc<Event>) -> bool {

    let mut is_relevant = true;

    match *event {
      Event::LoadPreference => self.load_preference(),
      Event::LoadSecret => self.load_secret(),
      _ => is_relevant = false,
    }

    is_relevant

  }

}

impl Subscriber for Loader {

  fn tx<'a>(&'a self) -> &'a Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Arc<Event>> {
    &self.rx
  }

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

