use config::{Preference, Secret};
use event::{Event, Recipient};
use state::Error;

/// Application state
#[derive(Clone, Debug)]
pub struct State {

  /// Errors to be displayed to the user
  pub error: Vec<Error>,

  /// Application preferences
  pub preference: Preference,

  /// Application secrets
  pub secret: Option<Secret>,

  /// Last key pressed by the user
  pub last_key_press: Option<char>,

  /// View path
  pub view: String,

}

impl State {

  /// Set preference value
  fn set_preference(&mut self, preference: &Preference) {
    self.preference = preference.clone()
  }

  /// Set secret value
  fn set_secret(&mut self, secret: &Secret) {
    self.secret = Some(secret.clone())
  }

  /// Set last key press value
  fn set_last_key_press(&mut self, character: &char) {
    self.last_key_press = Some(character.clone())
  }

  /// Add an error to be displayed to the user
  fn add_error<T>(&mut self, error: &T) where T: Clone + Into<Error> {
    self.error.push(error.clone().into())
  }

}

impl<'a> Recipient<&'a Event> for State {

  fn receive(&mut self, event: &'a Event) -> bool {

    let mut is_relevant = true;

    match *event {
      Event::LoadPreferenceOk(ref pref) => self.set_preference(pref),
      Event::LoadSecretOk(ref secret) => self.set_secret(secret),
      Event::KeyPress(ref character) => self.set_last_key_press(character),
      Event::LoadPreferenceErr(ref err)
      | Event::LoadSecretErr(ref err) => self.add_error(err),
      _ => is_relevant = false,
    }

    is_relevant

  }



}

impl Default for State {

  fn default() -> Self {
    State {
      error: Vec::new(),
      preference: Preference::default(),
      secret: None,
      last_key_press: None,
      view: "/".into(),
    }
  }

}

