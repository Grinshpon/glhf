use std::cell::RefCell;
use std::rc::Rc;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::GameError::WindowError;
use ggez::event::winit_event::*;

use glsp::prelude::*;

type RCtx = Rc<RefCell<ggez::Context>>;

struct Callbacks {
  load: Option<Root<GFn>>,
  update: Option<Root<GFn>>,
  draw: Option<Root<GFn>>,
}

impl Callbacks {
  fn new() -> Self {
    Callbacks {
      load: None,
      update: None,
      draw: None,
    }
  }

  fn unload(&mut self) {
    self.load = None;
    self.update = None;
    self.draw = None;
  }
}

struct MainState {
  pos_x: f32,
  callbacks: Callbacks,
  runtime: Runtime,
  context: RCtx,
}

impl MainState {
  fn new(runtime: Runtime, ctx: RCtx) -> ggez::GameResult<MainState> {
    let s = MainState {
      pos_x: 0.0,
      callbacks: Callbacks::new(),
      runtime: runtime,
      context: ctx,
    };
    Ok(s)
  }

  fn load(&mut self) -> Result<(),String> { //todo: actual error type
    let mut update: Option<Root<GFn>> = None;
    self.runtime.run(|| {
      update = match glsp::global::<_, Val>("glhf:update") {
        Ok(Val::GFn(update)) => Some(update),
        Ok(val) => {
          let msg = format!("invalid glhf:update value {}", val);
          return Ok(Err(String::from(&msg)))
        }
        Err(_) => return Ok(Err(String::from("glhf:update is not defined")))
      };
      Ok(Ok(()))
    }).unwrap()?;

    self.callbacks.update = update;
    Ok(())
  }

  fn update(&mut self) -> ggez::GameResult {
    if let Ok(ctx) = self.context.try_borrow_mut().as_mut() {
      let dt = ggez::timer::delta(ctx).as_secs_f64();
      let callbacks = &mut self.callbacks;
      self.runtime.run(|| {
        let _: Val = match glsp::call(callbacks.update.as_ref().unwrap(), (dt,)) {
          Ok(val) => val,
          Err(glsp_err) => {
            return Ok(Err(String::from(&glsp_err.to_string())))
          }
        };
        Ok(Ok(()))
      });
      self.pos_x = self.pos_x % 800.0 + 1.0;
      Ok(())
    }
    else {
      Err(WindowError("Multiple references to context".to_string()))
    }
  }

  fn draw(&mut self) -> ggez::GameResult {
    if let Ok(ctx) = self.context.try_borrow_mut().as_mut() {
      graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

      let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(self.pos_x, 380.0),
        100.0,
        2.0,
        graphics::WHITE,
      )?;
      graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

      graphics::present(ctx)?;
      Ok(())
    }
    else {
      //panic!("Aieee, something else is holding a reference to the context -- draw!!");
      Err(WindowError("Multiple references to context".to_string()))
    }
  }

  fn gc(&mut self) {
    self.runtime.run(|| {
      glsp::gc();
      Ok(())
    });
  }

  fn handle_input(&mut self, events_loop: &mut ggez::event::EventsLoop) -> ggez::GameResult {
    if let Ok(ctx) = self.context.try_borrow_mut().as_mut() {
      // Handle events. Refer to `winit` docs for more information.
      events_loop.poll_events(|event| {
        // This tells `ggez` to update it's internal states, should the event require that.
        // These include cursor position, view updating on resize, etc.
        ctx.process_event(&event);
        match event {
          Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => event::quit(ctx), //todo, have callback that returns bool (defaults to true)
            WindowEvent::KeyboardInput {
              input:
                KeyboardInput {
                  virtual_keycode: Some(keycode),
                  ..
                },
              ..
            } => match keycode {
              event::KeyCode::Escape => (), //event::quit(ctx),
              _ => (),
            },
            x => () //println!("Other window event fired: {:?}", x),
          },

          x => () //println!("Device event fired: {:?}", x),
        }
      });
      Ok(())
    }
    else {
      //panic!("Aieee, something else is holding a reference to the context -- draw!!");
      Err(WindowError("Multiple references to context".to_string()))
    }
  }
}

fn runtime_init() -> GResult<()> {
  //create bindings to ggez (TODO)
  glsp::bind_rfn("swap-bytes", &i32::swap_bytes)?; //placeholder

  //load main script
  glsp::load("main.glsp")?;
  Ok(())
}

fn run(events_loop: &mut ggez::event::EventsLoop, state: &mut MainState) -> ggez::GameResult {
  let mut continuing = true;

  match state.load() {
    Ok(_) => (),
    Err(s) => {eprintln!("{}",s); continuing = false;},
  }

  while continuing {
    if let Ok(ctx) = state.context.try_borrow_mut().as_mut() {
      // Tell the timer stuff a frame has happened.
      // Without this the FPS timer functions and such won't work.
      ctx.timer_context.tick();
      continuing = ctx.continuing;
    }
    else {
      continuing = false;
      return Err(WindowError("Multiple references to context".to_string()))
    }

    //handle inputs
    state.handle_input(events_loop)?;

    //update
    state.update()?;

    //draw
    state.draw()?;

    //collect garbage
    state.gc();

    //limit framerate (is this necessary? or use vsync?)
    ggez::timer::yield_now();
  }


  //Have to drop the Root<GFn>'s within the Gamelisp runtime environment or else it panics
  let callbacks = &mut state.callbacks;
  state.runtime.run(|| {
    callbacks.unload();
    Ok(())
  });

  Ok(())
}


pub fn main() -> ggez::GameResult { 
  //initialize gamelisp runtime
  let runtime = Runtime::new();
  runtime.run(runtime_init);

  //read configuration from main.glsp and apply to ggez context builder (TODO)

  //initialize ggez and state
  let cb = ggez::ContextBuilder::new("super_simple", "ggez");
  let (mut ctx, mut event_loop) = cb.build()?;
  let rctx = Rc::new(RefCell::new(ctx));
  let mut state = MainState::new(runtime, rctx)?;

  //run game loop
  run(&mut event_loop, &mut state)
}
