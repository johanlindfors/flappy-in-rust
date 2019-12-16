use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Color};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, State};

use crate::scenes::title::TitleScene;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

// === Scene Management ===

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context);
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
}

// Boxing/dynamic dispatch could be avoided here by defining an enum for all
// of your scenes, but that adds a bit of extra boilerplate - your choice!

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    scaler: ScreenScaler,
}

impl SceneManager {
    pub fn new(ctx: &mut Context) -> tetra::Result<SceneManager> {
        window::set_mouse_visible(ctx)?;
        let initial_scene = TitleScene::new(ctx)?;
        Ok(SceneManager {
            scenes: vec![Box::new(initial_scene)],
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                ScalingMode::ShowAllPixelPerfect,
            )?,
        })
    }
}

impl State for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());

        match self.scenes.last_mut() {
            Some(active_scene) => active_scene.draw(ctx),
            None => window::quit(ctx),
        }

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        graphics::draw(ctx, &self.scaler, Vec2::new(0.0, 0.0));

        Ok(())
    }
}
