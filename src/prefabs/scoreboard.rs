use tetra::graphics::text::{Font, Text};
use tetra::graphics::{DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

use crate::prefabs::button::Button;
use crate::SCREEN_WIDTH;

pub struct Scoreboard {
    game_over_texture: Texture,
    game_over_position: Vec2<f32>,
    game_over_origin: Vec2<f32>,

    scoreboard_texture: Texture,
    scoreboard_position: Vec2<f32>,
    scoreboard_origin: Vec2<f32>,

    score_text: Text,
    score_origin: Vec2<f32>,
    score: i32,

    highscore_text: Text,
    highscore_origin: Vec2<f32>,

    medal: Texture,

    pub button: Button,
}

impl Scoreboard {
    pub fn new(ctx: &mut Context) -> tetra::Result<Scoreboard> {
        let game_over_texture = Texture::new(ctx, "./resources/gameover.png")?;
        let scoreboard_texture = Texture::new(ctx, "./resources/scoreboard.png")?;

        Ok(Scoreboard {
            game_over_position: Vec2::new(SCREEN_WIDTH as f32 / 2.0, 100.0),
            game_over_origin: Vec2::new(
                game_over_texture.width() as f32 / 2.0,
                game_over_texture.height() as f32 / 2.0,
            ),
            game_over_texture,

            scoreboard_position: Vec2::new(SCREEN_WIDTH as f32 / 2.0, 200.0),
            scoreboard_origin: Vec2::new(
                scoreboard_texture.width() as f32 / 2.0,
                scoreboard_texture.height() as f32 / 2.0,
            ),
            scoreboard_texture,

            button: Button::new(ctx, Vec2::new(SCREEN_WIDTH as f32 / 2.0, 300.0))?,

            score_text: Text::new(
                "0",
                Font::vector(ctx, "./resources/font/flappy-font.ttf", 26.0)?,
            ),
            score_origin: Vec2::new(0.0, 0.0),
            highscore_text: Text::new(
                "0",
                Font::vector(ctx, "./resources/font/flappy-font.ttf", 26.0)?,
            ),

            highscore_origin: Vec2::new(0.0, 0.0),
            score: 0,

            medal: Texture::new(ctx, "./resources/medals.png")?,
        })
    }

    pub fn set_score(&mut self, ctx: &mut Context, score: i32, highscore: i32) {
        self.score = score;

        self.score_text.set_content(score.to_string());
        let bounds = self.score_text.get_bounds(ctx).unwrap();
        self.score_origin = Vec2::new(bounds.width, 0.0);

        self.highscore_text.set_content(highscore.to_string());
        let bounds = self.highscore_text.get_bounds(ctx).unwrap();
        self.highscore_origin = Vec2::new(bounds.width, 0.0);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.game_over_texture.draw(
            ctx,
            DrawParams::new()
                .position(self.game_over_position)
                .origin(self.game_over_origin),
        );
        self.scoreboard_texture.draw(
            ctx,
            DrawParams::new()
                .position(self.scoreboard_position)
                .origin(self.scoreboard_origin),
        );

        self.button.draw(ctx);

        self.score_text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(240.0, 176.0))
                .origin(self.score_origin),
        );

        self.highscore_text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(240.0, 222.0))
                .origin(self.highscore_origin),
        );

        if self.score >= 10 && self.score < 20 {
            self.medal.draw_region(
                ctx,
                Rectangle::new(0.0, 0.0, 44.0, 46.0),
                Vec2::new(58.0, 185.0),
            );
        } else if self.score >= 20 {
            self.medal.draw_region(
                ctx,
                Rectangle::new(0.0, 46.0, 44.0, 46.0),
                Vec2::new(58.0, 185.0),
            );
        }
    }
}
