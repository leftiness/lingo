extern crate lingo;

use std::thread;

use lingo::config;
use lingo::event::{self, Listener, Publisher, Subscriber};
use lingo::input;
use lingo::state;

/// Start Lingo
pub fn main() {

  let mut dispatcher = event::Dispatcher::default();
  let mut logger = event::Logger::default();
  let mut loader = config::Loader::with_dispatcher(&dispatcher);
  let mut broadcaster = input::Broadcaster::with_dispatcher(&dispatcher);
  let mut container = state::Container::with_dispatcher(&dispatcher);

  dispatcher.register(&logger);
  dispatcher.register(&loader);
  dispatcher.register(&container);

  let process = thread::spawn(move || Subscriber::listen(&mut dispatcher));

  thread::spawn(move || Subscriber::listen(&mut logger));
  thread::spawn(move || Subscriber::listen(&mut loader));
  thread::spawn(move || Subscriber::listen(&mut container));
  thread::spawn(move || broadcaster.listen());

  process.join().unwrap();

}

