use messages::{Request, Response};

/// A messageable struct can interact by passing messages
pub trait Messageable {

  /// Send a request and receive a response
  fn tell(&mut self, Request) -> Response;

}

