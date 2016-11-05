use messages::Request;

/// Messages responding to requests
#[derive(Debug)]
pub enum Response {

  /// The request makes no sense
  Balderdash(Request),

  /// Acknowledge a request
  Habadagus,

}

