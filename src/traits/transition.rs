/// Declare that a state can transition from another state
///
/// This trait is necessarily separate from std::convert::From because Rust
/// core blanket implements `From<T> for T`, which means that a new blanket
/// implementation `From<State<T>> for State<U> where U: From<T>` would
/// conflict where T = U. To avoid implementing From for each
/// (State<T>, State<U>) combination, I've created this trait.
///
/// States can and should still use From while the state machine struct itself
/// uses Transition.
pub trait Transition<T> {

  /// Implement the transition between states of the state machine
  fn from(t: T) -> Self;

}

