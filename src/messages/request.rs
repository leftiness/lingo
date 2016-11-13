use structs::{Message, Room};

/// Messages requesting action or data
#[derive(Debug)]
pub enum Request {

  /// Offer a chat room
  AddRoom(Room),

  /// Offer a chat message
  AddMessage(Message),

  /// Request access to the contents of the secret toml
  TellMeYourSecrets,

}

