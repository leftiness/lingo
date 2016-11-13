use messages::Request;
use structs::Secret;

/// Messages responding to requests
#[derive(Debug)]
pub enum Response<'a> {

  /// The request makes no sense
  Balderdash(Request),

  /// Acknowledge a request
  Habadagus,

  /// Declare a request impossible
  StarsCantDoIt,

  /// Share the contents of the secret toml
  DontTellAnyone(&'a Secret),

}

