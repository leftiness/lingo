extern crate lingo;

use std::thread;

use lingo::config::Loader;
use lingo::event::{Broadcaster, Event, Logger, Dispatcher};

/// Start Lingo
pub fn main() {

  let mut dispatcher = Dispatcher::new();

  let mut logger = Logger::with_dispatcher(&dispatcher);
  let mut loader = Loader::with_dispatcher(&dispatcher);

  let tx = dispatcher.tx().clone();

  dispatcher.register(logger.tx().clone());
  dispatcher.register(loader.tx().clone());

  let process = thread::spawn(move || dispatcher.listen());

  thread::spawn(move || logger.listen());
  thread::spawn(move || loader.listen());

  tx.send(Event::LoadPreference).unwrap();

  process.join().unwrap();

}

