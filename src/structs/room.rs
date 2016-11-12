use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::Into;

/// A chat room
#[derive(Debug, Eq, PartialEq)]
pub struct Room {

  /// Name of the chat room
  pub room_name: String,

}

impl Room {

  /// Create a new chat room
  pub fn new<T: Into<String>>(room_name: T) -> Self {
    Room { room_name: room_name.into() }
  }

}

impl Ord for Room {

  fn cmp(&self, other: &Room) -> Ordering {
    self.room_name.cmp(&other.room_name)
  }

}

impl PartialOrd for Room {

  fn partial_cmp(&self, other: &Room) -> Option<Ordering> {
    Some(self.cmp(other))
  }

}

