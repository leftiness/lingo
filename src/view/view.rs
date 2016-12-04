use std::io::{self, Write};

use state::State;
use view::Component;

/// Application views
#[derive(Clone, Debug)]
pub enum View {

  /// Main menu
  Menu,

}

impl Component for View {

  fn render<T: Write>(stdout: &mut T, state: &State) -> io::Result<()> {
    match state.view {
      View::Menu => write!(stdout, "{:?}", state),
    }
  }

}

impl Default for View {

  fn default() -> Self {
    View::Menu
  }

}

