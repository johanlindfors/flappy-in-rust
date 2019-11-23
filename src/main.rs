use tetra::{ContextBuilder};

mod systems;
mod scenes;
mod prefabs;

pub use systems::scenemanagement::{SceneManager};

pub const SCREEN_WIDTH: i32 = 288;
pub const SCREEN_HEIGHT: i32 = 505;
pub const GRAVITY: f32 = 9.1;
pub const SCROLL_SPEED: f32 = 3.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable(false)
        .build()?
        .run_with(SceneManager::new)
}
