/// Listeners listen for events
pub trait Listener {

  /// Block while listening for events
  fn listen(&mut self);

}

