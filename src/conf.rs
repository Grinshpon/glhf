use serde_derive::{Serialize,Deserialize};
use toml;

use ggez::ContextBuilder;
use ggez::conf;

#[derive(Serialize,Deserialize,Debug)]
pub struct Config {
  pub window_setup: Option<WindowSetup>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct WindowSetup {
  pub title: Option<String>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      window_setup: None,
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn from_toml() -> Option<Self> {
    let contents = match std::fs::read_to_string("conf.toml") {
      Ok(f) => f,
      Err(_) => return None,
    };
    match toml::from_str(&contents) {
      Ok(c) => Some(c),
      Err(e) => {eprintln!("{}",e); None},
    }
  }

  pub fn into_context_builder(self) -> ContextBuilder {
    let mut cb = ContextBuilder::new("glhf_app", "glhf");

    let mut setup = conf::WindowSetup::default();
    if let Some(ws) = self.window_setup {
      if let Some(t) = ws.title {
        setup = setup.title(&t);
      }
    }
    cb = cb.window_setup(setup);

    cb
  }
}
