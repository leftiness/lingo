use std::fmt::Display;

/// Supported XDG base directory types
#[derive(Debug)]
pub enum Xdg {
  Config,
  Cache,
}

impl Xdg {

  /// XDG environment variable
  pub fn env(&self) -> Option<&'static str> {
    match *self {
      Xdg::Config => option_env!("XDG_CONFIG_HOME"),
      Xdg::Cache => option_env!("XDG_CACHE_HOME"),
    }
  }

  /// Default path within the XDG base directory
  pub fn local(&self) -> &'static str {
    match *self {
      Xdg::Config => ".config",
      Xdg::Cache => ".cache",
    }
  }

  /// XDG environment variable or the default
  pub fn home(&self) -> String {
    match self.env() {
      Some(string) => string.to_owned(),
      None => format!("{}/{}", env!("HOME"), self.local()),
    }
  }

  /// Path to this application's directory within the XDG directory
  pub fn app(&self) -> String {
    format!("{}/{}", self.home(), env!("CARGO_PKG_NAME"))
  }

  /// XDG resource path for this application
  pub fn resource<T>(
    &self,
    resource_name: T
  ) -> String where T: Into<String> + Display {
    format!("{}/{}", self.app(), resource_name)
  }

}

