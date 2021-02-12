use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

use crate::systems::physics::{check_collision, PhysicsBody};
use crate::{SCREEN_WIDTH, SCROLL_SPEED};

pub struct Ground {
    texture: Texture,
    scroll_pos: f32,
    pub scroll: bool,
}

impl PhysicsBody for Ground {
    fn get_collision_rect(&mut self) -> Rectangle {
        Rectangle::new(0.0, 400.0, SCREEN_WIDTH as f32, 112.0)
    }

    fn collides_with(&mut self, obj: &Rectangle) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

impl Ground {
    pub fn new(ctx: &mut Context) -> tetra::Result<Ground> {
        Ok(Ground {
            texture: Texture::new(ctx, "./resources/ground.png")?,
            scroll_pos: 0.0,
            scroll: true,
        })
    }

    pub fn update(&mut self) {
        if self.scroll {
            self.scroll_pos = (self.scroll_pos - SCROLL_SPEED) % 335.0;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.texture.draw(ctx, Vec2::new(self.scroll_pos, 400.0));
        self.texture
            .draw(ctx, Vec2::new(self.scroll_pos + 335.0, 400.0));
    }
}
