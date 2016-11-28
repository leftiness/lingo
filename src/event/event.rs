use config::{self, Preference, Secret};
use state::State;

/// Events emitted to signal application events
#[derive(Clone, Debug)]
pub enum Event {

  /// The application state has been updated
  StateUpdate(State),

  /// Begin loading preferences
  LoadPreference,

  /// Preferences were successfully loaded
  LoadPreferenceOk(Preference),

  /// An error occurred while loading preferences
  LoadPreferenceErr(config::Error),

  /// Being loading secrets
  LoadSecret,

  /// Secrets were successfully loaded
  LoadSecretOk(Secret),

  /// An error occurred while loading secrets
  LoadSecretErr(config::Error),

  /// Keyboard input has been received
  KeyPress(char),

  /// User has signalled to close the application
  Quit,

}

