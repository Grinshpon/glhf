use ggez;
//use ggez::event;
use ggez::graphics;
//use ggez::nalgebra as na;
//use ggez::event::winit_event::*;

use glsp::prelude::*;

mod error;
use crate::error::*;

mod state;
use crate::state::*;

//mod shapes;
//use crate::shapes::*;

mod ggez_wrapper;
use ggez_wrapper as wrapper;

mod conf;
use crate::conf::*;

pub struct Context(pub ggez::Context);
impl RGlobal for Context { }

fn runtime_init(_state: &MainState) -> GResult<()> {
  //create bindings to ggez (TODO)
  glsp::bind_rfn("swap-bytes", &i32::swap_bytes)?; //placeholder

  //glsp::bind_rfn("new-circle", &Shape::new_circle)?;
  glsp::bind_rfn("new-circle", &wrapper::new_circle)?;
  glsp::bind_rfn("draw-shape", &wrapper::draw_shape)?;
  glsp::bind_global("draw-mode:fill", graphics::DrawMode::fill())?;
  glsp::bind_global("color:white", graphics::WHITE)?;

  //load main script
  glsp::load("main.glsp")?;
  Ok(())
}

fn run(events_loop: &mut ggez::event::EventsLoop, state: &mut MainState) -> GlhfResult {
  let mut continuing = true;

  state.load()?;

  while continuing {
    {
      let ctx = &mut Context::borrow_mut().0;
      // Tell the timer stuff a frame has happened.
      // Without this the FPS timer functions and such won't work.
      ctx.timer_context.tick();
      continuing = ctx.continuing;
      drop(ctx);
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
  //todo: give user option to specify id and author somehow (maybe read conf.toml myself, creating Conf and context names in the process?)
  let config = Config::from_toml();
  println!("{:?}",config);
  let cb = ggez::ContextBuilder::new("glhf_app", "glhf");
  let (ctx, mut event_loop) = cb.build()?;
  //let rctx = Rc::new(RefCell::new(ctx));
  let mut state = MainState::new();

  //run game loop
  let res = runtime.run(|| {
    glsp::add_rglobal(Context(ctx));

    runtime_init(&state)?;

    let res = run(&mut event_loop, &mut state);
    match res {
      Err(GlhfError::GlspError(err)) => Err(err),
      x => Ok(x),
    }
    //Ok(res)
  });
  match res {
    Some(ret) => ret,
    None => Err(GlhfError::Error(String::from("Closed due to Gamelisp errors"))),
  }
}
