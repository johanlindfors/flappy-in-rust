use tetra::ContextBuilder;

mod prefabs;
mod scenes;
mod systems;

pub use systems::scenemanagement::SceneManager;

pub const SCREEN_WIDTH: i32 = 288;
pub const SCREEN_HEIGHT: i32 = 505;
pub const GRAVITY: f32 = 9.1;
pub const SCROLL_SPEED: f32 = 3.0;
pub const FILE_NAME: &str = "highscore.txt";

fn main() -> tetra::Result {
    ContextBuilder::new("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable(false)
        .build()?
        .run(SceneManager::new)
}
