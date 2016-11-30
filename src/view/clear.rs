use std::io::{self, Write};

use termion::clear;
use termion::cursor;

use view::Component;

/// Clear the screen
#[derive(Debug)]
pub struct Clear {}

impl Component<()> for Clear {

  fn render<T: Write>(stdout: &mut T, _: &()) -> io::Result<()> {
    write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All)
  }

}

