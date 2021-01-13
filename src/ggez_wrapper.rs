use ggez::GameResult;
use ggez::graphics::{DrawMode,DrawParam,Color,Mesh};
use ggez::graphics;
use super::Context;

pub fn new_circle(ctx: &mut Context, mode: &DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: &Color) -> GameResult<Mesh> {
  Mesh::new_circle(&mut ctx.0, *mode, point, radius, tolerance, *color)
}

pub fn draw_shape(ctx: &mut Context, mesh: &Mesh, params: Option<&DrawParam>) -> GameResult {
  match params {
    Some(p) => graphics::draw(&mut ctx.0, mesh, *p),
    None    => graphics::draw(&mut ctx.0, mesh, DrawParam::default()),
  }
}
