use config::{Preference, Secret};
use event::Event;
use state::Error;

/// Application state
#[derive(Clone, Debug)]
pub struct State {

  /// Errors to be displayed to the user
  error: Vec<Error>,

  /// Application preferences
  preference: Preference,

  /// Application secrets
  secret: Option<Secret>,

  /// Last key pressed by the user
  last_key_press: Option<char>,

}

impl State {

  /// Update the state if it should be updated
  pub fn update(&mut self, event: &Event) -> bool {

    let mut should_update = true;

    match *event {
      Event::LoadPreferenceOk(ref pref) => self.set_preference(pref),
      Event::LoadSecretOk(ref secret) => self.set_secret(secret),
      Event::KeyPress(ref character) => self.set_last_key_press(character),
      Event::LoadPreferenceErr(ref err)
      | Event::LoadSecretErr(ref err) => self.add_error(err),
      _ => should_update = false,
    }

    should_update

  }

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

impl Default for State {

  fn default() -> Self {
    State {
      error: Vec::new(),
      preference: Preference::default(),
      secret: None,
      last_key_press: None,
    }
  }

}

