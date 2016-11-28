extern crate lingo;

use std::thread;

use lingo::config::Loader;
use lingo::event::{Dispatcher, Logger, Publisher, Subscriber};
use lingo::input::Broadcaster;

/// Start Lingo
pub fn main() {

  let mut dispatcher = Dispatcher::default();
  let mut logger = Logger::default();
  let mut loader = Loader::with_dispatcher(&dispatcher);

  let broadcaster = Broadcaster::with_dispatcher(&dispatcher);

  dispatcher.register(&logger);
  dispatcher.register(&loader);

  let process = thread::spawn(move || dispatcher.listen());

  thread::spawn(move || logger.listen());
  thread::spawn(move || loader.listen());
  thread::spawn(move || broadcaster.listen());

  process.join().unwrap();

}

