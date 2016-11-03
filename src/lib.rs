//! Lingo is a Hipchat client for your terminal

#![ deny
  ( missing_docs
  , missing_debug_implementations
  , trivial_casts
  , trivial_numeric_casts
  , unsafe_code
  , unstable_features
  , unused_import_braces
  , unused_qualifications
  )
]

extern crate rustc_serialize;
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

