use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use time;

use event::{Event, Recipient, Subscriber};
use xdg::Xdg;

/// File name for event log
const EVENT_LOG: &'static str = "event.log";

/// Log events
#[derive(Debug)]
pub struct Logger {

  /// Transmits messages to the receiver
  tx: Sender<Arc<Event>>,

  /// Receives messages from transmitters
  rx: Receiver<Arc<Event>>,

  /// Log file reference
  file: File,

}

impl Recipient for Logger {

  fn receive(&mut self, event: Arc<Event>) -> bool {

    let always_relevant = true;
    let time = time::now();
    let time = time.rfc3339();

    writeln!(self.file, "[{}] {:?}", time, event).unwrap();

    always_relevant

  }

}

impl Subscriber for Logger {

  fn tx<'a>(&'a self) -> &'a Sender<Arc<Event>> {
    &self.tx
  }

  fn rx<'a>(&'a self) -> &'a Receiver<Arc<Event>> {
    &self.rx
  }

}

impl Default for Logger {

  fn default() -> Self {

    let (tx, rx) = mpsc::channel();
    let dir = fs::create_dir_all(Xdg::Cache.app());

    let file = OpenOptions::new()
      .create(true)
      .write(true)
      .open(Xdg::Cache.resource(EVENT_LOG));

    Logger {
      tx: tx,
      rx: rx,
      file: dir.and(file).unwrap(),
    }

  }

}

