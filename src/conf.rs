use serde_derive::{Serialize,Deserialize};
use toml;

use ggez::ContextBuilder;
use ggez::conf;

/* For some reason I couldn't get ggez to read conf.toml by itself.
Also ggez's conf reading capabilities are kinda basic, so I'm making my own conf system.
For example, with pure ggez none of the fields are optional, you have to specify all them or the file won't be read.
Also documentation is a bit scarce.
I might contribute to improving ggez's system when I finish.
*/

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
    //todo: give user option to specify id and author somehow (maybe read conf.toml myself, creating Conf and context names in the process?)
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
