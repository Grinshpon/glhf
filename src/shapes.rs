use ggez::graphics::{DrawMode,Color};

pub enum Shape {
  Circle {
    mode: DrawMode,
    point: [f32; 2],
    radius: f32,
    tolerance: f32,
    color: Color,
  },
  DummyShape {
    dummy_data: [f32; 2],
  },
}

impl Shape {
  pub fn new_circle(mode: DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: Color) -> Self {
    Shape::Circle {
      mode,
      point,
      radius,
      tolerance,
      color,
    }
  }

  pub fn new_dummy_shape(dummy_data: [f32; 2]) -> Self {
    Shape::DummyShape { dummy_data }
  }
}
