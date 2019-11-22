use rand::{thread_rng, Rng};
use tetra::audio::Sound;
use tetra::graphics::ScreenScaling;
use tetra::graphics::{self, Color, DrawParams, Font, Text, Texture, Rectangle, Vec2};
use tetra::graphics::animation::Animation;
use tetra::input::{self, MouseButton};
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use std::f64;

const SCREEN_WIDTH: i32 = 288;
const SCREEN_HEIGHT: i32 = 505;
const GRAVITY: f32 = 9.1;

fn main() -> tetra::Result {
    ContextBuilder::new("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable(false)
        .quit_on_escape(true)
        .build()?
        .run_with(SceneManager::new)
}

// === Scene Management ===

trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context, dt: f64);
}

enum Transition {
    None,
    Push(Box<dyn Scene>),
    //Pop,
}

// Boxing/dynamic dispatch could be avoided here by defining an enum for all
// of your scenes, but that adds a bit of extra boilerplate - your choice!

struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    fn new(ctx: &mut Context) -> tetra::Result<SceneManager> {
        let initial_scene = TitleScene::new(ctx)?;
        graphics::set_scaling(ctx, ScreenScaling::ShowAllPixelPerfect);
        window::show_mouse(ctx);
        Ok(SceneManager {
            scenes: vec![Box::new(initial_scene)],
        })
    }
}

impl State for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                // Transition::Pop => {
                //     self.scenes.pop();
                // }
            },
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, dt: f64) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => active_scene.draw(ctx, dt),
            None => window::quit(ctx),
        }

        Ok(())
    }
}

// === Physics ===

trait PhysicsBody {
    fn get_collision_rect(&mut self) -> Rectangle;
    fn collides_with(&mut self, obj: &Rectangle) -> bool;
}

fn check_collision(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.x < rect2.x + rect2.width &&
    rect1.x + rect1.width > rect2.x &&
    rect1.y < rect2.y + rect2.height &&
    rect1.y + rect1.height > rect2.y
}

// === Parallax ground ===

struct Background {
    ground_texture: Texture,
    forest_texture: Texture,
    cityscape_texture: Texture,
    cloud_texture: Texture,

    ground_rect: Rectangle,
    forest_rect: Rectangle,
    cityscape_rect: Rectangle,
    cloud_rect: Rectangle,

    scroll: bool
}

impl PhysicsBody for Background {
    fn get_collision_rect(&mut self) -> Rectangle {
        Rectangle::new(0.0, 400.0, SCREEN_WIDTH as f32, 112.0)
    }

    fn collides_with(&mut self, obj: &Rectangle) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

impl Background {
    fn new(ctx: &mut Context) -> tetra::Result<Background> {
        Ok( Background {
            ground_texture: Texture::new(ctx, "./resources/ground.png")?,
            ground_rect: Rectangle::new(0.0, 0.0, 335.0, 112.0),

            forest_texture: Texture::new(ctx, "./resources/trees.png")?,
            forest_rect: Rectangle::new(0.0, 0.0, 335.0, 112.0),

            cityscape_texture: Texture::new(ctx, "./resources/cityscape.png")?,
            cityscape_rect: Rectangle::new(0.0, 0.0, 300.0, 43.0),

            cloud_texture: Texture::new(ctx, "./resources/clouds.png")?,
            cloud_rect: Rectangle::new(0.0, 0.0, 352.0, 100.0),

            scroll: true,
        })
    }

    fn update(&mut self) {
        if self.scroll {
            self.ground_rect.x += 4.0 ;
            self.forest_rect.x += 3.0 ;
            self.cityscape_rect.x += 2.0 ;
            self.cloud_rect.x += 1.0 ;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(ctx, &self.cloud_texture,
            DrawParams::new()
            .position(Vec2::new(0.0, 300.0))
            .clip(self.cloud_rect));

        graphics::draw(ctx, &self.cityscape_texture,
            DrawParams::new()
            .position(Vec2::new(0.0, 330.0))
            .clip(self.cityscape_rect));


        graphics::draw(ctx, &self.forest_texture,
            DrawParams::new()
            .position(Vec2::new(0.0, 360.0))
            .clip(self.forest_rect));

        graphics::draw(ctx, &self.ground_texture,
            DrawParams::new()
            .position(Vec2::new(0.0, 400.0))
            .clip(self.ground_rect));
    }
}

// === Bird ===

struct Bird {
    animation: Animation,
    rotation: f32,
    position: Vec2,
    velocity: Vec2,
    flap_counter: i32,
    flap_delta: f64,
    allow_gravity: bool,
    alive: bool,
}

impl PhysicsBody for Bird {
    fn get_collision_rect(&mut self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, 34.0, 24.0)
    }

    fn collides_with(&mut self, obj: &Rectangle) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

impl Bird {
    fn new(ctx: &mut Context) -> tetra::Result<Bird> {
        Ok(Bird {
            animation: Animation::new(
                Texture::new(ctx, "./resources/bird.png")?,
                Rectangle::row(0.0, 0.0, 34.0, 24.0).take(3).collect(),
                5,
            ),
            rotation: 0.0,
            position: Vec2::new(100.0, SCREEN_HEIGHT as f32/2.0),
            velocity: Vec2::new(0.0, 0.0),
            flap_counter: 0,
            flap_delta: 0.0,
            allow_gravity: false,
            alive: false,
        })
    }

    fn flap(&mut self) {
        if self.alive {
            self.velocity.y = -7.5;
            self.flap_counter = 6;
            self.tween_rotation();
        }
    }

    fn kill(&mut self) {
        if self.alive && self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
        }
        self.alive = false;
    }

    fn reset(&mut self) {
        self.velocity = Vec2::new(0.0, 0.0);
        self.position = Vec2::new(100.0, SCREEN_HEIGHT as f32 / 2.0);
        self.rotation = 0.0;
        self.flap_delta = 0.0;
        self.alive = true;
    }

    fn tween_rotation(&mut self) {
        let distance = (-1.0 - self.rotation) as f64;
        self.flap_delta = distance.abs() / self.flap_counter as f64;
    }

    fn update(&mut self) {
        if self.alive {
            self.animation.tick();
        }

        if self.allow_gravity {
            self.velocity.y = self.velocity.y + GRAVITY / 30.0;
            self.position.y = self.position.y + self.velocity.y;
            if self.position.y <= 12.0 {
                self.position.y = 12.0;
                self.velocity.y = 0.0;
            }

            if self.flap_counter > 0 {
                self.rotation -= self.flap_delta as f32;
                self.flap_counter -= 1;
            } if self.rotation < 1.3 {
                self.rotation += 0.05;
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context ) {
        graphics::draw(
            ctx,
            &self.animation,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(17.0, 12.0))
                .rotation(self.rotation)
        );
    }
}

// === Pipes ===

struct Pipe {
    position: Vec2,
    source_rect: Rectangle,
}

impl Pipe {
    fn new(position: Vec2, source_rect: Rectangle) -> tetra::Result<Pipe> {
        Ok(Pipe {
            position: position,
            source_rect: source_rect,
        })
    }

    fn draw(&mut self, ctx: &mut Context, position: Vec2, texture: &Texture) {
        graphics::draw(ctx, texture, DrawParams::new()
                .position(Vec2::new(self.position.x + position.x, self.position.y + position.y))
                .clip(self.source_rect));
    }
}

impl PhysicsBody for Pipe {
    fn get_collision_rect(&mut self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, 54.0, 320.0)
    }

    fn collides_with(&mut self, obj: &Rectangle) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

struct PipeGroup {
    position: Vec2,
    top_pipe: Pipe,
    bottom_pipe: Pipe,
    alive: bool,
    enabled: bool,
    has_scored: bool,
}

impl PipeGroup {
    fn new() -> tetra::Result<PipeGroup> {
        Ok(PipeGroup {
            position: Vec2::new(0.0, 0.0),
            top_pipe: Pipe::new(Vec2::new(0.0, 0.0), Rectangle::new(0.0, 0.0, 54.0, 320.0))?,
            bottom_pipe: Pipe::new(Vec2::new(0.0, 440.0), Rectangle::new(54.0, 0.0, 54.0, 320.0))?,
            alive: false,
            enabled: false,
            has_scored: false,
        })
    }

    fn update(&mut self, _ctx: &mut Context) {
        if self.alive && self.enabled {
            self.position.x -= 4.0;
        }
        if self.position.x < -54.0 {
            self.alive = false;
            self.enabled = false;
        }
    }

    fn draw(&mut self, ctx: &mut Context, texture: &Texture) {
        self.top_pipe.draw(ctx, self.position, texture);
        self.bottom_pipe.draw(ctx, self.position, texture);
    }

    fn reset(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y - 160.0;
        self.alive = true;
        self.enabled = true;
        self.has_scored = false;
    }
}

impl PhysicsBody for PipeGroup {
    fn get_collision_rect(&mut self) -> Rectangle {
        Rectangle::new(0.0, 0.0, 0.0, 0.0)
    }

    fn collides_with(&mut self, obj: &Rectangle) -> bool {
        let relative_rect = Rectangle::new(obj.x - self.position.x - 27.0,
                                           obj.y - self.position.y - 12.0,
                                           obj.width,
                                           obj.height);
        self.top_pipe.collides_with(&relative_rect) ||
        self.bottom_pipe.collides_with(&relative_rect)
    }
}

struct Scoreboard {
    game_over_texture: Texture,
    game_over_position: Vec2,
    game_over_origin: Vec2,

    scoreboard_texture: Texture,
    scoreboard_position: Vec2,
    scoreboard_origin: Vec2,

    button: Button,

    score_text: Text,
    score_origin: Vec2,
    score: i32,

    highscore_text: Text,
    highscore_origin: Vec2,

    medal: Texture,
}

impl Scoreboard {
    fn new(ctx: &mut Context) -> tetra::Result<Scoreboard> {
        let game_over_texture = Texture::new(ctx, "./resources/gameover.png")?;
        let scoreboard_texture = Texture::new(ctx, "./resources/scoreboard.png")?;
        Ok(Scoreboard {
            game_over_position: Vec2::new(SCREEN_WIDTH as f32/ 2.0, 100.0),
            game_over_origin:Vec2::new(game_over_texture.width() as f32/ 2.0,
                                       game_over_texture.height() as f32/ 2.0),
            game_over_texture: game_over_texture,

            scoreboard_position: Vec2::new(SCREEN_WIDTH as f32/ 2.0, 200.0),
            scoreboard_origin:Vec2::new(scoreboard_texture.width() as f32/ 2.0,
                                        scoreboard_texture.height() as f32/ 2.0),
            scoreboard_texture: scoreboard_texture,

            button: Button::new(ctx, Vec2::new(SCREEN_WIDTH as f32/ 2.0, 300.0))?,

            score_text: Text::new("0", Font::default(), 26.0),
            score_origin: Vec2::new(0.0, 0.0),
            highscore_text: Text::new("0", Font::default(), 26.0),
            highscore_origin: Vec2::new(0.0, 0.0),
            score: 0,

            medal: Texture::new(ctx, "./resources/medals.png")?,
        })
    }

    fn set_score(&mut self, ctx: &mut Context, score: i32, highscore: i32) {
        self.score = score;

        self.score_text.set_content(score.to_string());
        let bounds = self.score_text.get_bounds(ctx).unwrap();
        self.score_origin = Vec2::new(-bounds.width, 0.0);

        self.highscore_text.set_content(highscore.to_string());
        let bounds = self.highscore_text.get_bounds(ctx).unwrap();
        self.highscore_origin = Vec2::new(-bounds.width, 0.0);
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(ctx, &self.game_over_texture, DrawParams::new()
                    .position(self.game_over_position)
                    .origin(self.game_over_origin));
        graphics::draw(ctx, &self.scoreboard_texture, DrawParams::new()
                    .position(self.scoreboard_position)
                    .origin(self.scoreboard_origin));

        self.button.draw(ctx);

        graphics::draw(ctx, &self.score_text, DrawParams::new()
                .position(Vec2::new(215.0, 176.0))
                .origin(self.score_origin));

        graphics::draw(ctx, &self.highscore_text, DrawParams::new()
                .position(Vec2::new(215.0, 222.0))
                .origin(self.highscore_origin));

        if self.score >= 10 && self.score < 20 {
            graphics::draw(ctx, &self.medal, DrawParams::new()
                .position(Vec2::new(58.0, 185.0))
                .clip(Rectangle::new(0.0, 0.0, 44.0, 46.0)));
        } else if self.score >= 20 {
            graphics::draw(ctx, &self.medal, DrawParams::new()
                .position(Vec2::new(58.0, 185.0))
                .clip(Rectangle::new(0.0, 46.0, 44.0, 46.0)));
        }
    }
}

struct PipeGenerator {
    counter: i32,
    enabled: bool
}

impl PipeGenerator {
    fn new() -> tetra::Result<PipeGenerator> {
        Ok(PipeGenerator {
            counter: 0,
            enabled: false,
        })
    }

    fn start(&mut self) {
        self.enabled = true;
    }

    fn stop(&mut self) {
        self.enabled = false;
    }

    fn should_spawn_pipe(&mut self) -> bool {
        if self.enabled {
            self.counter += 1;
            if self.counter >= 80 {
                self.counter = 0;
                return true;
            }
        }

        false
    }
}

// === Button ===

struct Button {
    texture: Texture,
    rect: Rectangle,
}

impl Button {
    fn new(ctx: &mut Context, centered_position: Vec2) -> tetra::Result<Button> {
        let texture = Texture::new(ctx, "./resources/start-button.png")?;
        let rect = Rectangle::new(
            centered_position.x - texture.width() as f32 / 2.0,
            centered_position.y - texture.height() as f32 / 2.0,
            texture.width() as f32,
            texture.height() as f32
        );

        Ok(Button {
            texture: texture,
            rect: rect,
        })
    }

    fn contains(&mut self, point: Vec2) -> bool {
        point.x >= self.rect.x &&
        point.x <= (self.rect.x + self.rect.width) &&
        point.y >= self.rect.y &&
        point.y <= (self.rect.y + self.rect.height)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(ctx, &self.texture, Vec2::new(self.rect.x, self.rect.y));
    }
}

// === Title Scene ===

struct TitleScene {
    sky_texture: Texture,
    title: Texture,
    bird: Animation,
    background: Background,
    button: Button,
}

impl TitleScene {
    fn new(ctx: &mut Context) -> tetra::Result<TitleScene> {

        Ok(TitleScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            title: Texture::new(ctx, "./resources/title.png")?,

            bird: Animation::new(
                Texture::new(ctx, "./resources/bird.png")?,
                Rectangle::row(0.0, 0.0, 34.0, 24.0).take(3).collect(),
                5,
            ),
            background: Background::new(ctx)?,

            button: Button::new(ctx, Vec2::new(SCREEN_WIDTH as f32/2.0, 300.0))?,
        })
    }
}

impl Scene for TitleScene {

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.background.update();

        let mouse_position = input::get_mouse_position(ctx);
        if input::is_mouse_button_down(ctx, MouseButton::Left) &&  self.button.contains(mouse_position) {
            Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) {
        graphics::draw(ctx, &self.sky_texture, Vec2::new(0.0, 0.0));

        self.background.draw(ctx);

        graphics::draw(ctx, &self.bird, Vec2::new(230.0,105.0));

        graphics::draw(ctx, &self.title, Vec2::new(30.0, 100.0));

        self.button.draw(ctx);
    }
}

// === Game Scene ===

struct GameScene {
    sky_texture: Texture,
    background: Background,
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
    fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        let mut bird = Bird::new(ctx)?;
        bird.reset();

        Ok(GameScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            background: Background::new(ctx)?,
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

    fn check_for_collisions(&mut self, ctx: &mut Context) -> tetra::Result {
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
            self.pipe_hit_sound.play(ctx)?;
            self.bird.kill();

            self.pipe_generator.stop();
            self.background.scroll = false;

            for pipe_group in &mut self.pipes {
                pipe_group.enabled = false;
            }
        }

        if !self.game_over && self.bird.collides_with(&self.background.get_collision_rect()) {
            self.ground_hit_sound.play(ctx)?;
            self.bird.allow_gravity = false;
            self.background.scroll = false;

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

        Ok(())
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
