use hyper::method::Method;

use config::Secret;

/// Supported Hipchat API endpoints
#[derive(Debug)]
pub enum Api {

  /// List non-archived rooms for this group
  ///
  /// https://www.hipchat.com/docs/apiv2/method/get_all_rooms
  GetAllRooms,

}

impl Api {

  /// HTTP method for this API endpoint
  pub fn method(&self) -> Method {
    match *self {
      Api::GetAllRooms => Method::Get,
    }
  }

  /// Hipchat API's URL endpoint
  pub fn endpoint(&self) -> &'static str {
    match *self {
      Api::GetAllRooms => "/v2/room",
    }
  }

  /// Full URL for a Hipchat API request
  pub fn url(&self, secret: &Secret) -> String {
    format!(
      "https://{}{}",
      secret.origin,
      self.endpoint()
    )
  }

}

