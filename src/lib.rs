//! Lingo is a Hipchat client for your terminal

#![ deny
  ( dead_code
  , missing_docs
  , missing_debug_implementations
  , trivial_casts
  , trivial_numeric_casts
  , unreachable_code
  , unsafe_code
  , unstable_features
  , unused_assignments
  , unused_extern_crates
  , unused_features
  , unused_import_braces
  , unused_imports
  , unused_qualifications
  , unused_variables
  )
]

extern crate rustc_serialize;
extern crate time;
extern crate toml;

/// Error enums
pub mod errors;

/// Messages passed to states
pub mod messages;

/// States in the state machine
pub mod states;

/// Various utility structs
pub mod structs;

/// Various utility traits
pub mod traits;

