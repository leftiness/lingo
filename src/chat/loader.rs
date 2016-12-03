use std::io::Read;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use hyper::client::{Client, Response};
use hyper::header::{Authorization, Bearer};

use rustc_serialize::{Decodable, json};

use config::Secret;
use event::{Dispatcher, Event, Publisher, Recipient, Subscriber};
use chat::{Api, Error, Rooms};

/// Loads initial data from Hipchat
#[derive(Debug)]
pub struct Loader {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

  /// Transmits messages to the dispatcher
  dx: Sender<Event>,

  /// Hyper client for outbound HTTP requests
  client: Client,

}

impl Loader {

  /// Make an HTTP request
  fn request(
    &self,
    secret: &Secret,
    api: Api,
  ) -> Result<Response, Error> {

    let request = self.client.request(api.method(), &api.url(secret));
    let auth = Authorization(Bearer { token: secret.token.clone() });
    let response = try!(request.header(auth).send());

    Ok(response)

  }

  /// Load available rooms
  fn load_rooms(&self, secret: &Secret) {

    let api = Api::GetAllRooms;
    let decoder = decode_response::<Rooms>;

    let event = match self.request(secret, api).and_then(decoder) {
      Ok(rooms) => Event::LoadRoomsOk(rooms.into()),
      Err(err) => Event::LoadRoomsErr(err.into()),
    };

    self.dx.send(event).unwrap();

  }

}

impl Publisher for Loader {

  fn with_dispatcher(dispatcher: &Dispatcher) -> Self {

    let (tx, rx) = mpsc::channel();

    Loader {
      tx: tx,
      rx: rx,
      dx: dispatcher.tx().clone(),
      client: Client::new(),
    }

  }

}

impl Recipient for Loader {

  fn receive(&mut self, event: Arc<Event>) -> bool {

    let mut is_relevant = true;

    match *event {
      Event::LoadRoomsWithSecret(ref secret) => self.load_rooms(secret),
      _ => is_relevant = false,
    }

    is_relevant

  }

}

impl Subscriber for Loader {

  fn tx<'a>(&'a self) -> &'a Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Arc<Event>> {
    &self.rx
  }

}

/// Load and decode a toml from the provided path
fn decode_response<T>(mut res: Response) -> Result<T, Error>
  where T: Decodable {

  let mut json_string = String::new();

  try!(res.read_to_string(&mut json_string));

  let decoded = try!(json::decode::<T>(&json_string));

  Ok(decoded)

}

