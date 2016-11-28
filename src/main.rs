extern crate lingo;

use std::thread;

use lingo::config::Loader;
use lingo::event::{Dispatcher, Event, Logger, Publisher, Subscriber};

/// Start Lingo
pub fn main() {

  let mut dispatcher = Dispatcher::default();
  let mut logger = Logger::default();
  let mut loader = Loader::with_dispatcher(&dispatcher);

  let tx = dispatcher.tx().clone();

  dispatcher.register(&logger);
  dispatcher.register(&loader);

  let process = thread::spawn(move || dispatcher.listen());

  thread::spawn(move || logger.listen());
  thread::spawn(move || loader.listen());

  tx.send(Event::LoadPreference).unwrap();

  process.join().unwrap();

}

