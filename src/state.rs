use ggez;
use ggez::event;
use ggez::graphics;
//use ggez::nalgebra as na;
use ggez::event::winit_event::*;

use glsp::prelude::*;

use crate::error::*;
use super::Context;

pub struct Callbacks {
  load: Option<Root<GFn>>,
  update: Option<Root<GFn>>,
  draw: Option<Root<GFn>>,
}

impl Callbacks {
  pub fn new() -> Self {
    Callbacks {
      load: None,
      update: None,
      draw: None,
    }
  }

  pub fn unload(&mut self) {
    self.load = None;
    self.update = None;
    self.draw = None;
  }
}

pub struct MainState {
  pub pos_x: f32,
  pub callbacks: Callbacks,
}

impl MainState {
  pub fn new() -> MainState {
    MainState {
      pos_x: 0.0,
      callbacks: Callbacks::new(),
    }
  }

  pub fn load(&mut self) -> GlhfResult {
    //get load function
    self.callbacks.load = match glsp::global::<_, Val>("glhf:load") {
      Ok(Val::GFn(update)) => Some(update),
      Ok(val) => {
        let msg = format!("invalid glhf:load value {}", val);
        return Err(GlhfError::from(GError::from_str(&msg)))
      }
      Err(_) => return Err(GlhfError::from(GError::from_str("glhf:load is not defined")))
    };
    //get update function
    self.callbacks.update = match glsp::global::<_, Val>("glhf:update") {
      Ok(Val::GFn(update)) => Some(update),
      Ok(val) => {
        let msg = format!("invalid glhf:update value {}", val);
        return Err(GlhfError::from(GError::from_str(&msg)))
      }
      Err(_) => return Err(GlhfError::from(GError::from_str("glhf:update is not defined")))
    };
    //get draw function
    self.callbacks.draw = match glsp::global::<_, Val>("glhf:draw") {
      Ok(Val::GFn(update)) => Some(update),
      Ok(val) => {
        let msg = format!("invalid glhf:draw value {}", val);
        return Err(GlhfError::from(GError::from_str(&msg)))
      }
      Err(_) => return Err(GlhfError::from(GError::from_str("glhf:draw is not defined")))
    };

    //call load function
    let _: Val = match glsp::call(self.callbacks.load.as_ref().unwrap(), ()) {
      Ok(val) => val,
      Err(glsp_err) => {
        return Err(GlhfError::from(glsp_err))
      }
    };

    Ok(())
  }

  //should I separata these functions that don't need state into a seperate struct/module?
  fn delta_time() -> f32 {
    let ctx = &mut Context::borrow_mut().0;
    ggez::timer::delta(ctx).as_secs_f32()
    //context has to be dropped before gamelisp functions are called or else we'll get a double borrow
  }

  pub fn update(&mut self) -> GlhfResult {
    let dt = Self::delta_time();
    let _: Val = match glsp::call(self.callbacks.update.as_ref().unwrap(), (dt,)) {
      Ok(val) => val,
      Err(glsp_err) => {
        return Err(GlhfError::from(glsp_err))
      }
    };
    self.pos_x = self.pos_x % 800.0 + 1.0;
    Ok(())
  }

  fn clear() {
    let ctx = &mut Context::borrow_mut().0;
    graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
  }

  fn present() -> GlhfResult {
    let ctx = &mut Context::borrow_mut().0;
    graphics::present(ctx)?;
    Ok(())
  }

  pub fn draw(&mut self) -> GlhfResult {
    Self::clear();
    let _: Val = match glsp::call(self.callbacks.draw.as_ref().unwrap(), ()) {
      Ok(val) => val,
      Err(glsp_err) => {
        return Err(GlhfError::from(glsp_err))
      }
    };
    Self::present()
  }

  pub fn handle_input(&mut self, events_loop: &mut ggez::event::EventsLoop) -> GlhfResult {
    let ctx = &mut Context::borrow_mut().0;
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
          _x => () //println!("Other window event fired: {:?}", x),
        },

        _x => () //println!("Device event fired: {:?}", x),
      }
    });
    Ok(())
  }
}
