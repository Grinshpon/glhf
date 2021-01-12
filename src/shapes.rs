use ggez::graphics::{DrawMode,Color};

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
  pub fn new_circle(mode: &DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: &Color) -> Self {
    Shape::Circle {
      mode: *mode,
      point,
      radius,
      tolerance,
      color: *color,
    }
  }
}
