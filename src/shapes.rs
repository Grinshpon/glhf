use ggez::GameResult;
use ggez::graphics::{DrawMode,Color,Mesh};
use super::Context;

pub enum Shape {
  Circle {
    mode: DrawMode,
    point: [f32; 2],
    radius: f32,
    tolerance: f32,
    color: Color,
  },
}

impl Shape {
  pub fn new_circle_old(mode: &DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: &Color) -> Self {
    Shape::Circle {
      mode: *mode,
      point,
      radius,
      tolerance,
      color: *color,
    }
  }

  pub fn new_circle(ctx: &mut Context, mode: &DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: &Color) -> GameResult<Mesh> {
    Mesh::new_circle(&mut ctx.0, *mode, point, radius, tolerance, *color)
  }
}
