use ggez;
use glsp::prelude::*;

#[derive(Debug)]
pub enum GlhfError {
  GgezError(ggez::GameError),
  GlspError(GError),
  Error(String),
}

impl From<ggez::GameError> for GlhfError {
  fn from(err: ggez::GameError) -> Self {
    GlhfError::GgezError(err)
  }
}
impl From<GError> for GlhfError {
  fn from(err: GError) -> Self {
    GlhfError::GlspError(err)
  }
}

pub type GlhfResult<T=()> = Result<T, GlhfError>;

pub fn from_ggez_result<T>(res: ggez::GameResult<T>) -> GlhfResult<T> {
  match res {
    Ok(x) => Ok(x),
    Err(err) => Err(GlhfError::from(err)),
  }
}
