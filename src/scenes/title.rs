use std::time::Duration;
use tetra::graphics::animation::Animation;
use tetra::graphics::{Rectangle, Texture};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::Context;

use crate::prefabs::background::Background;
use crate::prefabs::button::Button;
use crate::prefabs::ground::Ground;
use crate::scenes::{game::GameScene, Scene, Transition};
use crate::SCREEN_WIDTH;

pub struct TitleScene {
    sky_texture: Texture,
    title: Texture,
    bird: Animation,
    background: Background,
    ground: Ground,
    button: Button,
}

impl TitleScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<TitleScene> {
        Ok(TitleScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            title: Texture::new(ctx, "./resources/title.png")?,

            bird: Animation::new(
                Texture::new(ctx, "./resources/bird.png")?,
                Rectangle::row(0.0, 0.0, 34.0, 24.0).take(3).collect(),
                Duration::from_secs_f64(0.2),
            ),
            background: Background::new(ctx)?,
            ground: Ground::new(ctx)?,

            button: Button::new(ctx, Vec2::new(SCREEN_WIDTH as f32 / 2.0, 300.0))?,
        })
    }
}

impl Scene for TitleScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.background.update();
        self.ground.update();

        let mouse_position = input::get_mouse_position(ctx);
        if input::is_mouse_button_down(ctx, MouseButton::Left)
            && self.button.contains(mouse_position)
        {
            Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
        } else if input::is_key_pressed(ctx, Key::Escape) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.sky_texture.draw(ctx, Vec2::new(0.0, 0.0));

        self.background.draw(ctx);
        self.ground.draw(ctx);

        self.bird.draw(ctx, Vec2::new(230.0, 105.0));

        self.title.draw(ctx, Vec2::new(30.0, 100.0));

        self.button.draw(ctx);
    }
}
