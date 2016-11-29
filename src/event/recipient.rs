use std::sync::Arc;

use event::Event;

/// Recipients receive events
pub trait Recipient<T = Arc<Event>> {

  /// Receive an event, returning whether the event was relevant
  fn receive(&mut self, T) -> bool;

}

