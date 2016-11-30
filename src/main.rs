extern crate lingo;

use std::thread;

use lingo::config;
use lingo::event::{self, Listener, Publisher, Subscriber};
use lingo::input;
use lingo::state;
use lingo::view;

/// Start Lingo
pub fn main() {

  let mut dispatcher = event::Dispatcher::default();
  let mut logger = event::Logger::default();
  let mut loader = config::Loader::with_dispatcher(&dispatcher);
  let mut broadcaster = input::Broadcaster::with_dispatcher(&dispatcher);
  let mut state = state::Container::with_dispatcher(&dispatcher);
  let mut view = view::Container::with_dispatcher(&dispatcher);

  dispatcher.register(&logger);
  dispatcher.register(&loader);
  dispatcher.register(&state);
  dispatcher.register(&view);

  let threads = vec![
    thread::spawn(move || Subscriber::listen(&mut dispatcher)),
    thread::spawn(move || Subscriber::listen(&mut logger)),
    thread::spawn(move || Subscriber::listen(&mut loader)),
    thread::spawn(move || Subscriber::listen(&mut state)),
    thread::spawn(move || Subscriber::listen(&mut view)),
    thread::spawn(move || broadcaster.listen()),
  ];

  for thread in threads {
    thread.join().unwrap();
  }

}

