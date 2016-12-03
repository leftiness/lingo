use std::collections::BTreeSet;

use chat::{self, Room};
use config::{self, Preference, Secret};
use state::{self, State};

/// Events emitted to signal application events
#[derive(Clone, Debug)]
pub enum Event {

  /// The application state has been updated
  StateUpdate(State),

  /// There is an error in the application state
  StateErr(state::Error),

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

  /// Begin loading Hipchat rooms
  LoadRooms,

  /// Make the request to the Hipchat API
  LoadRoomsWithSecret(Secret),

  /// Hipchat rooms were successfully loaded
  LoadRoomsOk(BTreeSet<Room>),

  /// An error occurred while loading Hipchat rooms
  LoadRoomsErr(chat::Error),

}

impl AsRef<Event> for Event {

  fn as_ref<'a>(&'a self) -> &'a Self {
    &self
  }

}

