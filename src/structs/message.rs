use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::Into;

use time::Tm;

/// A chat message
#[derive(Debug, Eq, PartialEq)]
pub struct Message {

  /// Name of the chat room
  pub room_name: String,

  /// Name of the sender
  pub sender_name: String,

  /// Time the message was sent
  pub sent_time: Tm,

  /// Text of the message
  pub message_text: String,

}

impl Message {

  /// Create a new chat message
  pub fn new<T>(
    room_name: T,
    sender_name: T,
    sent_time: Tm,
    message_text: T,
  ) -> Self where T: Into<String> {
    Message {
      room_name: room_name.into(),
      sender_name: sender_name.into(),
      sent_time: sent_time,
      message_text: message_text.into(),
    }
  }

}

impl Ord for Message {

  fn cmp(&self, other: &Message) -> Ordering {
    self.sent_time.cmp(&other.sent_time)
  }

}

impl PartialOrd for Message {

  fn partial_cmp(&self, other: &Message) -> Option<Ordering> {
    Some(self.cmp(other))
  }

}

