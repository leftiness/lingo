use event::Dispatcher;

/// Publishers send events
pub trait Publisher<T> {

  /// Create a new publisher with a dispatcher
  fn with_dispatcher(&Dispatcher<T>) -> Self;

}

