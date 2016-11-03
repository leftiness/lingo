use messages::Request;

/// Messages responding to requests
#[derive(Debug)]
pub enum Response {

  /// The request makes no sense
  Balderdash(Request),

  /// Declare that the requested resource is missing
  CantFindIt,

  /// Acknowledge a request successfully
  Thanks,

  /// Provide the path to the secret toml
  SecretPath(String),

}

