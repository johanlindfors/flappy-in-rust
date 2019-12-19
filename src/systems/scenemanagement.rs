use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Color};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, State};

use crate::scenes::{title::TitleScene, Scene, Transition};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

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
