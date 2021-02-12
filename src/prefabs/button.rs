use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

pub struct Button {
    texture: Texture,
    rect: Rectangle,
}

impl Button {
    pub fn new(ctx: &mut Context, centered_position: Vec2<f32>) -> tetra::Result<Button> {
        let texture = Texture::new(ctx, "./resources/start-button.png")?;
        let rect = Rectangle::new(
            centered_position.x - texture.width() as f32 / 2.0,
            centered_position.y - texture.height() as f32 / 2.0,
            texture.width() as f32,
            texture.height() as f32,
        );

        Ok(Button { texture, rect })
    }

    pub fn contains(&mut self, point: Vec2<f32>) -> bool {
        point.x >= self.rect.x
            && point.x <= (self.rect.x + self.rect.width)
            && point.y >= self.rect.y
            && point.y <= (self.rect.y + self.rect.height)
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.texture.draw(ctx, Vec2::new(self.rect.x, self.rect.y));
    }
}
