pub mod game;
pub mod title;
use tetra::Context;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context);
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
}
