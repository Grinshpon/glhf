use serde_derive::{Serialize,Deserialize};
use toml;

#[derive(Serialize,Deserialize)]
pub struct Config {
  pub window_setup: Option<WindowSetup>,
}

#[derive(Serialize,Deserialize)]
pub struct WindowSetup {
  pub title: Option<String>,
}

impl Config {
  pub fn from_toml() -> Option<Self> {
    let contents = match std::fs::read_to_string("conf.toml") {
      Ok(f) => f,
      Err(_) => return None,
    };
    let config: Option<Config> = match toml::from_str(&contents) {
      Ok(c) => Some(c),
      Err(e) => {eprintln!("{}",e); None},
    };
    config
  }
}
