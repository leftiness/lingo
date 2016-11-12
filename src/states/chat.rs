use std::collections::BTreeSet;
use std::convert::From;

use actors::Config;
use messages::{Request, Response};
use states::Load;
use structs::{Message, Room};
use traits::Messageable;

/// State which receives messages
#[derive(Debug)]
pub struct Chat {

  /// Config actor
  pub config: Config,

  /// Active chat rooms
  pub rooms: BTreeSet<Room>,

  /// Chat messages
  pub messages: BTreeSet<Message>,

}

impl Chat {

  /// Handle a request to add a room
  fn add_room(&mut self, room: Room) -> Response {
    self.rooms.insert(room);
    Response::Habadagus
  }

  /// Handle a request to add a message
  fn add_message(&mut self, message: Message) -> Response {
    self.messages.insert(message);
    Response::Habadagus
  }

}

impl Messageable for Chat {

  fn tell(&mut self, request: Request) -> Response {

    match request {
      Request::AddRoom(room) => self.add_room(room),
      Request::AddMessage(message) => self.add_message(message),
    }

  }

}

impl From<Load> for Chat {

  fn from(load: Load) -> Self {
    Chat {
      config: load.config,
      rooms: BTreeSet::new(),
      messages: BTreeSet::new(),
    }
  }

}

