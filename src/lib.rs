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

extern crate hyper;
extern crate rustc_serialize;
extern crate termion;
extern crate time;
extern crate toml;

/// Interact with the Hipchat API
pub mod chat;

/// Load configuration files
pub mod config;

/// Transmit and receive events
pub mod event;

/// Handle user input
pub mod input;

/// Store application state
pub mod state;

/// Render the application interface
pub mod view;

/// Utility functions for getting XDG-compliant paths
mod xdg;

