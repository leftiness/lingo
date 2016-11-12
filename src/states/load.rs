use std::convert::From;

use actors::Config;
use messages::{Request, Response};
use states::Start;
use traits::Messageable;

/// State which loads configurations
#[derive(Debug)]
pub struct Load {

  /// Config actor
  pub config: Config,

}

impl Messageable for Load {

  fn tell(&mut self, request: Request) -> Response {
    Response::Balderdash(request)
  }

}

impl From<Start> for Load {

  fn from(_: Start) -> Load {
    Load { config: Config::new() }
  }

}

