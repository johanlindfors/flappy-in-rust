use tetra::graphics::{self, Texture, Text, Font, Color, Vec2, DrawParams};
use tetra::{Context};
use tetra::input::{self, Key, MouseButton};
use tetra::audio::Sound;

use rand::{thread_rng, Rng};

use crate::systems::scenemanagement::{Scene, Transition};
use crate::{SCREEN_WIDTH};
use crate::prefabs::background::{Background};
use crate::prefabs::ground::{Ground};
use crate::prefabs::bird::{Bird};
use crate::prefabs::pipes::{PipeGroup, PipeGenerator};
use crate::prefabs::scoreboard::{Scoreboard};
use crate::systems::physics::PhysicsBody;

pub struct GameScene {
    sky_texture: Texture,
    background: Background,
    ground: Ground,
    pipes_texture: Texture,

    instructions: Texture,
    get_ready: Texture,

    bird: Bird,

    flap_sound: Sound,
    ground_hit_sound: Sound,
    pipe_hit_sound: Sound,
    score_sound: Sound,

    score: i32,
    highscore: i32,
    score_text: Text,

    is_mouse_down: bool,
    instructions_visible: bool,

    pipes: Vec<PipeGroup>,
    game_over: bool,
    pipe_generator: PipeGenerator,

    scoreboard: Scoreboard,
}

impl GameScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        let mut bird = Bird::new(ctx)?;
        bird.reset();

        Ok(GameScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            background: Background::new(ctx)?,
            ground: Ground::new(ctx)?,
            pipes_texture: Texture::new(ctx, "./resources/pipes.png")?,
            get_ready: Texture::new(ctx, "./resources/get-ready.png")?,
            instructions: Texture::new(ctx, "./resources/instructions.png")?,

            bird: bird,

            flap_sound: Sound::new("./resources/flap.wav")?,
            ground_hit_sound: Sound::new("./resources/ground-hit.wav")?,
            pipe_hit_sound: Sound::new("./resources/pipe-hit.wav")?,
            score_sound: Sound::new("./resources/score.wav")?,

            score: 0,
            highscore: 0,
            score_text: Text::new("0", Font::default(), 36.0),

            is_mouse_down: true,
            instructions_visible: true,
            pipes: Vec::new(),
            game_over: false,
            pipe_generator: PipeGenerator::new()?,

            scoreboard: Scoreboard::new(ctx)?,
        })
    }

    fn reset(&mut self) {
        self.instructions_visible = true;
        self.pipes.clear();
        self.background.scroll = true;
        self.ground.scroll = true;
        self.bird.reset();
        self.score = 0;
        self.game_over = false;
        self.score_text.set_content(self.score.to_string());
    }

    fn start_game(&mut self) {
        if self.instructions_visible {
            self.instructions_visible = false;
        }
        self.bird.allow_gravity = true;

        self.pipe_generator.start();
    }

    fn check_for_collisions(&mut self, ctx: &mut Context) {
        let mut bird_died = false;
        if self.bird.alive {
            for pipe_group in &mut self.pipes {
                if pipe_group.collides_with(&self.bird.get_collision_rect()) {
                    bird_died = true;
                    continue;
                }
            }
        }

        if bird_died {
            assert!(self.pipe_hit_sound.play(ctx).is_ok());
            self.bird.kill();

            self.pipe_generator.stop();
            self.background.scroll = false;
            self.ground.scroll = false;

            for pipe_group in &mut self.pipes {
                pipe_group.enabled = false;
            }
        }

        if !self.game_over && self.bird.collides_with(&self.ground.get_collision_rect()) {
            assert!(self.ground_hit_sound.play(ctx).is_ok());
            self.bird.allow_gravity = false;
            self.background.scroll = false;
            self.ground.scroll = false;

            self.game_over = true;
            self.pipe_generator.stop();

            if self.score >= self.highscore {
                self.highscore = self.score;
            }
            self.scoreboard.set_score(ctx, self.score, self.highscore);

            for pipe_group in &mut self.pipes {
                pipe_group.enabled = false;
            }
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.bird.update();

        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            if !self.is_mouse_down {
                let mouse_position = input::get_mouse_position(ctx);
                if self.instructions_visible {
                    self.start_game();
                } else if self.game_over && self.scoreboard.button.contains(mouse_position) {
                    self.reset();
                }
                if self.bird.alive && !self.instructions_visible {
                    self.flap_sound.play(ctx)?;
                    self.bird.flap();
                }
                self.is_mouse_down = true;
            }
        } else {
            self.is_mouse_down = false;
        }

        if !self.game_over {
            for pipe_group in &mut self.pipes {
                if !pipe_group.has_scored && pipe_group.position.x + 27.0 <= self.bird.position.x {
                    pipe_group.has_scored = true;
                    self.score_sound.play(ctx)?;
                    self.score += 1;
                    self.score_text.set_content(self.score.to_string());
                }
                pipe_group.update(ctx);
            }

            self.background.update();
            self.ground.update();

            self.check_for_collisions(ctx);

            if self.pipe_generator.should_spawn_pipe() {
                let mut rng = thread_rng();
                let y: f32 = rng.gen_range(-100.0, 100.0);

                for pipe_group in &mut self.pipes {
                    if !pipe_group.alive {
                        pipe_group.reset(SCREEN_WIDTH as f32, y);
                        return Ok(Transition::None);
                    }
                }
                let mut pipe_group = PipeGroup::new()?;
                pipe_group.reset(SCREEN_WIDTH as f32, y);
                self.pipes.push(pipe_group);
            }
        }

        if input::is_key_pressed(ctx, Key::Escape) {
            return Ok(Transition::Pop);
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.sky_texture, Vec2::new(0.0, 0.0));

        self.background.draw(ctx);

        if self.instructions_visible {
            graphics::draw(ctx, &self.instructions, DrawParams::new()
                .position(Vec2::new(SCREEN_WIDTH as f32/2.0, 325.0))
                .origin(Vec2::new(self.instructions.width() as f32/2.0,self.instructions.height() as f32/2.0)));
            graphics::draw(ctx, &self.get_ready, DrawParams::new()
                .position(Vec2::new(SCREEN_WIDTH as f32/2.0, 100.0))
                .origin(Vec2::new(self.get_ready.width() as f32/2.0,self.get_ready.height() as f32/2.0)));
        }

        for pipe_group in &mut self.pipes {
            pipe_group.draw(ctx, &self.pipes_texture);
        }

        self.ground.draw(ctx);

        if !self.game_over {
            let text_bounds = self.score_text.get_bounds(ctx).unwrap();
            graphics::draw(ctx, &self.score_text, DrawParams::new()
                .position(Vec2::new(SCREEN_WIDTH as f32 / 2.0, 10.0))
                .origin(Vec2::new(text_bounds.width / 2.0, 0.0)));
        } else {
            self.scoreboard.draw(ctx);
        }

        self.bird.draw(ctx);
    }
}
