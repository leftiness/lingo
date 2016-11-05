/// Declare that a state can transition from another state
///
/// This trait is necessarily separate from std::convert::From because Rust
/// core blanket implements `From<T> for T`, which means that a new blanket
/// implementation `From<State<T>> for State<U> where U: From<T>` would
/// conflict where T = U. To avoid implementing From for each
/// (State<T>, State<U>) combination, I've created this trait.
pub trait Transition<T> {

  /// Implement the transition between states
  fn from(t: T) -> Self;

}

