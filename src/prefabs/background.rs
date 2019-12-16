use tetra::graphics::{self, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

use crate::SCROLL_SPEED;

pub struct Background {
    forest_texture: Texture,
    cityscape_texture: Texture,
    cloud_texture: Texture,

    forest_rect: Rectangle,
    cityscape_rect: Rectangle,
    cloud_rect: Rectangle,

    pub scroll: bool,
}

impl Background {
    pub fn new(ctx: &mut Context) -> tetra::Result<Background> {
        Ok(Background {
            forest_texture: Texture::new(ctx, "./resources/trees.png")?,
            forest_rect: Rectangle::new(0.0, 0.0, 335.0, 112.0),

            cityscape_texture: Texture::new(ctx, "./resources/cityscape.png")?,
            cityscape_rect: Rectangle::new(0.0, 0.0, 300.0, 43.0),

            cloud_texture: Texture::new(ctx, "./resources/clouds.png")?,
            cloud_rect: Rectangle::new(0.0, 0.0, 352.0, 100.0),

            scroll: true,
        })
    }

    pub fn update(&mut self) {
        if self.scroll {
            self.forest_rect.x += SCROLL_SPEED * 0.75;
            self.cityscape_rect.x += SCROLL_SPEED * 0.5;
            self.cloud_rect.x += SCROLL_SPEED * 0.25;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.cloud_texture,
            DrawParams::new()
                .position(Vec2::new(0.0, 300.0))
                .clip(self.cloud_rect),
        );

        graphics::draw(
            ctx,
            &self.cityscape_texture,
            DrawParams::new()
                .position(Vec2::new(0.0, 330.0))
                .clip(self.cityscape_rect),
        );

        graphics::draw(
            ctx,
            &self.forest_texture,
            DrawParams::new()
                .position(Vec2::new(0.0, 360.0))
                .clip(self.forest_rect),
        );
    }
}
