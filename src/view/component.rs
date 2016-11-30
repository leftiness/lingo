use std::io::{self, Write};

use state::State;

/// Components render state
pub trait Component<T = State> {

  /// Render the state
  fn render<U: Write>(&mut U, &T) -> io::Result<()>;

}

