use std::cell::RefCell;
use std::rc::Rc;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event::winit_event::*;

use glsp::prelude::*;

mod error;
use crate::error::*;

mod state;
use crate::state::*;

mod shapes;
use crate::shapes::*;

fn runtime_init(state: &MainState) -> GResult<()> {
  //create bindings to ggez (TODO)
  glsp::bind_rfn("swap-bytes", &i32::swap_bytes)?; //placeholder

  //let new_circle = |mode: graphics::DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: graphics::Color| -> Shape {
    //state.new_circle(mode,point,radius,tolerance,color).unwrap()
    //Circle {
      //mode,point,radius,tolerance,color
    //}
  //};
  //glsp::bind_rfn("new-circle", &Shape::new_circle)?;
  glsp::bind_rfn("new-dummy-shape", &Shape::new_dummy_shape)?;

  //load main script
  glsp::load("main.glsp")?;
  Ok(())
}

fn run(events_loop: &mut ggez::event::EventsLoop, state: &mut MainState) -> GlhfResult {
  let mut continuing = true;

  state.load()?;

  while continuing {
    if let Ok(ctx) = state.context.try_borrow_mut().as_mut() {
      // Tell the timer stuff a frame has happened.
      // Without this the FPS timer functions and such won't work.
      ctx.timer_context.tick();
      continuing = ctx.continuing;
    }
    else {
      //continuing = false;
      return Err(GlhfError::Error("Multiple references to context".to_string()))
    }

    //handle inputs
    state.handle_input(events_loop)?;

    //update
    state.update()?;

    //draw
    state.draw()?;

    //collect garbage
    glsp::gc();

    //limit framerate (is this necessary? or use vsync?)
    ggez::timer::yield_now();
  }

  //Have to drop the Root<GFn>'s within the Gamelisp runtime environment or else it panics
  state.callbacks.unload();

  Ok(())
}


pub fn main() -> GlhfResult { 
  //initialize gamelisp runtime
  let runtime = Runtime::new();

  //read configuration from conf.glsp and apply to ggez context builder (TODO)

  //initialize ggez and state
  let cb = ggez::ContextBuilder::new("super_simple", "ggez");
  let (ctx, mut event_loop) = cb.build()?;
  let rctx = Rc::new(RefCell::new(ctx));
  let mut state = MainState::new(rctx);

  //run game loop
  let res = runtime.run(|| {
    runtime_init(&state)?;

    let res = run(&mut event_loop, &mut state);
    match res {
      Err(GlhfError::GlspError(err)) => Err(err),
      x => Ok(x),
    }
  });
  match res {
    Some(ret) => ret,
    None => Err(GlhfError::Error(String::from("Closed due to Gamelisp errors"))),
  }
}
