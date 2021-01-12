#![feature(once_cell)] // 1.48.0-nightly (2020-08-28 d006f5734f49625c34d6)
use std::{lazy::SyncLazy, sync::Mutex};

//use std::cell::RefCell;
//use std::rc::Rc;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event::winit_event::*;
use ggez::Context;
use ggez::event::EventsLoop;

use glsp::prelude::*;

mod error;
use crate::error::*;

mod state;
use crate::state::*;

pub static CONTEXT: SyncLazy<Mutex<(Context, EventsLoop)>> = SyncLazy::new(|| Mutex::new(ggez::ContextBuilder::new("super_simple", "ggez").build().unwrap()));

fn runtime_init(state: &MainState) -> GResult<()> {
  //create bindings to ggez (TODO)
  glsp::bind_rfn("swap-bytes", &i32::swap_bytes)?; //placeholder

  /*
  let new_circle = |mode: graphics::DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: graphics::Color| -> graphics::Mesh {
    state.new_circle(mode,point,radius,tolerance,color).unwrap()
  };
  glsp::bind_rfn("new-circle", &new_circle)?;
  */

  //load main script
  glsp::load("main.glsp")?;
  Ok(())
}

fn run(state: &mut MainState) -> GlhfResult {
  let mut continuing = true;

  state.load()?;

  while continuing {
    {
      let ctx = &mut CONTEXT.lock().unwrap().0;
      // Tell the timer stuff a frame has happened.
      // Without this the FPS timer functions and such won't work.
      ctx.timer_context.tick();
      continuing = ctx.continuing;
    }

    //handle inputs
    state.handle_input()?;

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
  //let cb = ggez::ContextBuilder::new("super_simple", "ggez");
  //let (ctx, mut event_loop) = cb.build()?;
  let mut state = MainState::new();

  //run game loop
  let res = runtime.run(|| {
    runtime_init(&state)?;

    let res = run(&mut state);
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
