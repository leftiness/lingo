use std::collections::BTreeSet;
use std::cmp::Ordering;

/// A Hipchat room
#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable)]
pub struct Room {

  /// What is the room's ID?
  pub id: i32,

  /// Is the room archived?
  pub is_archived: bool,

  /// What is the room's name?
  pub name: String,

}

impl Ord for Room {

  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(&other.id)
  }

}

impl PartialOrd for Room {

  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }

}

/// A collection of Hipchat rooms
#[derive(Clone, Debug, RustcDecodable)]
pub struct Rooms {

  /// Hipchat rooms
  pub items: Vec<Room>,

}

impl From<Rooms> for BTreeSet<Room> {

  fn from(rooms: Rooms) -> BTreeSet<Room> {
    rooms.items.into_iter().collect()
  }

}

