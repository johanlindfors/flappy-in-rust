use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::Context;

use crate::SCROLL_SPEED;

pub struct Background {
    forest_texture: Texture,
    cityscape_texture: Texture,
    cloud_texture: Texture,

    forest_pos: f32,
    cityscape_pos: f32,
    cloud_pos: f32,

    pub scroll: bool,
}

impl Background {
    pub fn new(ctx: &mut Context) -> tetra::Result<Background> {
        Ok(Background {
            forest_texture: Texture::new(ctx, "./resources/trees.png")?,
            forest_pos: 0.0,

            cityscape_texture: Texture::new(ctx, "./resources/cityscape.png")?,
            cityscape_pos: 0.0,

            cloud_texture: Texture::new(ctx, "./resources/clouds.png")?,
            cloud_pos: 0.0,

            scroll: true,
        })
    }

    pub fn update(&mut self) {
        if self.scroll {
            self.forest_pos = (self.forest_pos - SCROLL_SPEED * 0.75) % 335.0;
            self.cityscape_pos = (self.cityscape_pos - SCROLL_SPEED * 0.5) % 300.0;
            self.cloud_pos = (self.cloud_pos - SCROLL_SPEED * 0.25) % 352.0;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.cloud_texture
            .draw(ctx, Vec2::new(self.cloud_pos, 300.0));
        self.cloud_texture
            .draw(ctx, Vec2::new(self.cloud_pos + 352.0, 300.0));

        self.cityscape_texture
            .draw(ctx, Vec2::new(self.cityscape_pos, 330.0));
        self.cityscape_texture
            .draw(ctx, Vec2::new(self.cityscape_pos + 300.0, 330.0));

        self.forest_texture
            .draw(ctx, Vec2::new(self.forest_pos, 360.0));
        self.forest_texture
            .draw(ctx, Vec2::new(self.forest_pos + 335.0, 360.0));
    }
}
