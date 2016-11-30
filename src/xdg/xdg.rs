use std::fmt::Display;

/// Supported XDG base directory types
#[derive(Debug)]
pub enum Xdg {
  Config,
}

impl Xdg {

  /// XDG environment variable
  pub fn env(&self) -> Option<&'static str> {
    match *self {
      Xdg::Config => option_env!("XDG_CONFIG_HOME"),
    }
  }

  /// Default path within the XDG base directory
  pub fn local(&self) -> &'static str {
    match *self {
      Xdg::Config => ".config",
    }
  }

  /// XDG environment variable or the default
  pub fn home(&self) -> String {
    match self.env() {
      Some(string) => string.to_owned(),
      None => format!("{}/{}", env!("HOME"), self.local()),
    }
  }

  /// XDG resource path
  pub fn resource<T>(
    &self,
    resource_name: T
  ) -> String where T: Into<String> + Display {
    format!("{}/{}", self.home(), resource_name)
  }

}

