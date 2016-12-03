use std::error::Error as ErrorTrait;
use std::io;

use hyper;

use rustc_serialize::json;

/// An error occured with the Hipchat API
#[derive(Clone, Debug)]
pub enum Error {

  /// JSON text was invalid
  InvalidText(String),

  /// Failed to make an HTTP request
  FailedToMakeHttpRequest(String),

  /// Failed to decode HTTP response
  FailedToDecodeHttpResponse(String),

}

impl From<hyper::Error> for Error {

  fn from(err: hyper::Error) -> Self {
    Error::FailedToMakeHttpRequest(err.description().to_owned())
  }

}

impl From<io::Error> for Error {

  fn from(err: io::Error) -> Self {
    Error::FailedToMakeHttpRequest(err.description().to_owned())
  }

}

impl From<json::DecoderError> for Error {

  fn from(err: json::DecoderError) -> Self {
    Error::FailedToDecodeHttpResponse(err.description().to_owned())
  }

}

