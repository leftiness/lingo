/// Secrets for connecting to Hipchat
#[derive(Clone, Debug, RustcDecodable)]
pub struct Secret {

  /// API token
  pub token: String,

  /// Origin URL
  pub origin: String,

}

