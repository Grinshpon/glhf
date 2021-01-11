use std::cell::RefCell;
use std::rc::Rc;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event::winit_event::*;

use glsp::prelude::*;

use crate::error::*;

pub type RCtx = Rc<RefCell<ggez::Context>>;

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
  pub context: RCtx,
}

impl MainState {
  pub fn new(ctx: RCtx) -> MainState {
    MainState {
      pos_x: 0.0,
      callbacks: Callbacks::new(),
      context: ctx,
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

  pub fn update(&mut self) -> GlhfResult {
    if let Ok(ctx) = self.context.try_borrow_mut().as_mut() {
      let dt = ggez::timer::delta(ctx).as_secs_f64();
      let _: Val = match glsp::call(self.callbacks.update.as_ref().unwrap(), (dt,)) {
        Ok(val) => val,
        Err(glsp_err) => {
          return Err(GlhfError::from(glsp_err))
        }
      };
      self.pos_x = self.pos_x % 800.0 + 1.0;
      Ok(())
    }
    else {
      Err(GlhfError::Error("Multiple references to context".to_string()))
    }
  }

  pub fn draw(&mut self) -> GlhfResult {
    if let Ok(ctx) = self.context.try_borrow_mut().as_mut() {
      graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

      let _: Val = match glsp::call(self.callbacks.draw.as_ref().unwrap(), ()) {
        Ok(val) => val,
        Err(glsp_err) => {
          return Err(GlhfError::from(glsp_err))
        }
      };

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
      Err(GlhfError::Error("Multiple references to context".to_string()))
    }
  }

  pub fn handle_input(&mut self, events_loop: &mut ggez::event::EventsLoop) -> GlhfResult {
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
            _x => () //println!("Other window event fired: {:?}", x),
          },

          _x => () //println!("Device event fired: {:?}", x),
        }
      });
      Ok(())
    }
    else {
      Err(GlhfError::Error("Multiple references to context".to_string()))
    }
  }

  fn new_circle(&mut self, mode: graphics::DrawMode, point: [f32; 2], radius: f32, tolerance: f32, color: graphics::Color) -> GlhfResult<graphics::Mesh> {
    if let Ok(ctx) = self.context.try_borrow_mut().as_mut() {
      from_ggez_result(graphics::Mesh::new_circle(ctx,mode,point,radius,tolerance,color))
    }
    else {
      Err(GlhfError::Error("Multiple references to context".to_string()))
    }
  }
}
