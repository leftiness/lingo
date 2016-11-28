use event::{Event, Dispatcher};

/// Publishers send events
pub trait Publisher<T = Event> {

  /// Create a new publisher with a dispatcher
  fn with_dispatcher(&Dispatcher<T>) -> Self;

}

