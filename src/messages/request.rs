/// Messages requesting action or data
#[derive(Debug)]
pub enum Request {

  /// Offer a path to the secret toml
  AcceptSecretPath(String),

}

