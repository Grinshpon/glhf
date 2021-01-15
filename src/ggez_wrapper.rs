use glsp::prelude::*;

use ggez::GameResult;
use ggez::graphics::{DrawMode,DrawParam,Color,Mesh};
use ggez::graphics;
use super::Context;

macro_rules! bind_rfns {
  ($(($name:expr, $func:expr)),+ $(,)?) => {
    $(
      glsp::bind_rfn($name, &$func)?;
    )+
  }
}

pub fn make_bindings() -> GResult<()> {
  glsp::bind_rfn("swap-bytes", &i32::swap_bytes)?; //placeholder

  // Shapes
  bind_rfns!(
    ("draw-shape", draw_shape),
    ("new-line", new_line),
    ("new-circle", new_circle),
    ("new-ellipse", new_ellipse),
    ("new-polyline", new_polyline),
    ("new-polygon", new_polygon),
    ("new-rectangle", new_rectangle),
  );

  // DrawMode
  // should I get rid of `draw-mode:...` and just have `fill/stroke`?
  glsp::bind_global("fill", graphics::DrawMode::fill())?;
  glsp::bind_rfn("stroke", &graphics::DrawMode::stroke)?;

  // Color
  glsp::bind_global("color:white", graphics::WHITE)?;
  glsp::bind_global("color:black", graphics::BLACK)?;
  glsp::bind_rfn("color", &new_color)?;

  Ok(())
}

// Shapes
pub fn draw_shape(ctx: &mut Context, mesh: &Mesh, params: Option<&DrawParam>) -> GameResult {
  match params {
    Some(p) => graphics::draw(&mut ctx.0, mesh, *p),
    None    => graphics::draw(&mut ctx.0, mesh, DrawParam::default()),
  }
}

pub fn new_line(ctx: &mut Context, points: &[[f32;2]], width: f32, color: &Color) -> GameResult<Mesh> {
  Mesh::new_line(&mut ctx.0, points, width, *color)
}

pub fn new_circle(ctx: &mut Context, mode: &DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: &Color) -> GameResult<Mesh> {
  Mesh::new_circle(&mut ctx.0, *mode, point, radius, tolerance, *color)
}

pub fn new_ellipse(ctx: &mut Context, mode: &DrawMode, point: [f32;2], radius1: f32, radius2: f32, tolerance: f32, color: &Color) -> GameResult<Mesh> {
  Mesh::new_ellipse(&mut ctx.0, *mode, point, radius1, radius2, tolerance, *color)
}

pub fn new_polyline(ctx: &mut Context,mode: &DrawMode,points: &[[f32;2]],color: &Color) -> GameResult<Mesh> {
  Mesh::new_polyline(&mut ctx.0, *mode, points, *color)
} 

pub fn new_polygon(ctx: &mut Context,mode: &DrawMode,points: &[[f32;2]],color: &Color) -> GameResult<Mesh> {
  Mesh::new_polygon(&mut ctx.0, *mode, points, *color)
}

// todo: bind Rect api (omg there's so much stuff to bind, it's insane)
pub fn new_rectangle(ctx: &mut Context,mode: &DrawMode,bounds: &graphics::Rect,color: &Color) -> GameResult<Mesh> {
  Mesh::new_rectangle(&mut ctx.0, *mode, *bounds, *color)
}

// Color
pub fn new_color(r: f32, g: f32, b: f32, a: Option<f32>) -> graphics::Color {
  graphics::Color::new(r,g,b,a.unwrap_or(1.0))
}
