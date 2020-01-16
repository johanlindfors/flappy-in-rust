use tetra::graphics::{self, Color};
use tetra::window;
use tetra::{Context, State};

use crate::scenes::{title::TitleScene, Scene, Transition};

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    pub fn new(ctx: &mut Context) -> tetra::Result<SceneManager> {
        match window::set_mouse_visible(ctx, true) {
            Ok(_) => {
                let initial_scene = TitleScene::new(ctx)?;
                Ok(SceneManager {
                    scenes: vec![Box::new(initial_scene)],
                })
            }
            Err(e) => panic!("Couldn't show mouse: {:?}", e),
        }
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
        graphics::clear(ctx, Color::BLACK);
        match self.scenes.last_mut() {
            Some(active_scene) => active_scene.draw(ctx),
            None => window::quit(ctx),
        }

        Ok(())
    }
}
