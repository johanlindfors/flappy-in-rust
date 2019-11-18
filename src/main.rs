use rand::{self, Rng};
use tetra::audio::Sound;
use tetra::graphics::ScreenScaling;
use tetra::graphics::{self, Color, DrawParams, Font, Text, Texture, Rectangle, Vec2};
use tetra::graphics::animation::Animation;
use tetra::input::{self, Key, MouseButton};
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
    Pop,
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
                Transition::Pop => {
                    self.scenes.pop();
                }
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
        })
    }

    fn flap(&mut self) {
        self.velocity.y = -7.5;
        self.flap_counter = 6;
        self.tween_rotation();
    }

    fn reset(&mut self) {
        self.velocity = Vec2::new(0.0, 0.0);
        self.position = Vec2::new(100.0, SCREEN_HEIGHT as f32/2.0);
    }

    fn tween_rotation(&mut self) {
        let distance = (-1.0 - self.rotation) as f64;
        self.flap_delta = distance.abs() / self.flap_counter as f64;
    }

    fn update(&mut self) {
        if self.allow_gravity {
            self.animation.tick();

            self.velocity.y = self.velocity.y + GRAVITY / 30.0;
            self.position.y = self.position.y + self.velocity.y;

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
    texture: Texture,
}

impl Pipe {
    fn new(ctx: &mut Context, position: Vec2, source_rect: Rectangle) -> tetra::Result<Pipe> {
        Ok(Pipe {
            position: position,
            source_rect: source_rect,
            texture: Texture::new(ctx, "./resources/pipes.png")?
        })
    }

    fn update(&mut self) {

    }

    fn draw(&mut self, ctx: &mut Context, position: Vec2) {
        graphics::draw(ctx, &self.texture, DrawParams::new()
                .position(Vec2::new(self.position.x + position.x, self.position.y + position.y))
                .clip(self.source_rect));
    }
}

struct PipeGroup {
    position: Vec2,
    top_pipe: Pipe,
    bottom_pipe: Pipe,
}

impl PipeGroup {
    fn new(ctx: &mut Context) -> tetra::Result<PipeGroup> {
        Ok(PipeGroup {
            position: Vec2::new(0.0, 0.0),
            top_pipe: Pipe::new(ctx, Vec2::new(0.0, 0.0), Rectangle::new(0.0, 0.0, 54.0, 320.0))?,
            bottom_pipe: Pipe::new(ctx, Vec2::new(0.0, 440.0), Rectangle::new(54.0, 0.0, 54.0, 320.0))?,
        })
    }

    fn update(&mut self, ctx: &mut Context) {
        self.top_pipe.update();
        self.bottom_pipe.update();
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.top_pipe.draw(ctx, self.position);
        self.bottom_pipe.draw(ctx, self.position);
    }

    fn reset(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}

// === Title Scene ===

struct TitleScene {
    sky_texture: Texture,
    title: Texture,
    start: Texture, 
    bird: Animation,
    background: Background,
    start_rect: Rectangle,
}

impl TitleScene {
    fn new(ctx: &mut Context) -> tetra::Result<TitleScene> {
        let button_texture = Texture::new(ctx, "./resources/start-button.png")?;
        let start_rect = Rectangle::new(
            SCREEN_WIDTH as f32/2.0 - button_texture.width() as f32 / 2.0, 
            300.0 - button_texture.height() as f32 / 2.0,
            button_texture.width() as f32,
            button_texture.height() as f32    
        );

        Ok(TitleScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            title: Texture::new(ctx, "./resources/title.png")?,
            start: button_texture,
            
            bird: Animation::new(
                Texture::new(ctx, "./resources/bird.png")?,
                Rectangle::row(0.0, 0.0, 34.0, 24.0).take(3).collect(),
                5,
            ),
            background: Background::new(ctx)?,
            start_rect: start_rect
        })
    }

    fn button_contains(&mut self, point: Vec2) -> bool {
        point.x >= self.start_rect.x &&
        point.x <= (self.start_rect.x + self.start_rect.width) &&
        point.y >= self.start_rect.y &&
        point.y <= (self.start_rect.y + self.start_rect.height)
    }
}

impl Scene for TitleScene {

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.background.update();

        let mouse_position = input::get_mouse_position(ctx);
        if input::is_mouse_button_down(ctx, MouseButton::Left) &&  self.button_contains(mouse_position) {
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
        graphics::draw(ctx, &self.start, Vec2::new(self.start_rect.x, self.start_rect.y));
    }
}

// === Game Scene ===

struct GameScene {
    sky_texture: Texture,
    background: Background,

    instructions: Texture,
    get_ready: Texture,

    bird: Bird,
    
    flap_sound: Sound,
    ground_hit_sound: Sound,
    pipe_hit_sound: Sound,
    score_sound: Sound,
    ouch_sound: Sound,

    drop_timer: i32,
    move_timer: i32,
    score: i32,
    score_text: Text,

    is_mouse_down: bool,
    instructions_visible: bool,

    pipes: Vec<PipeGroup>,
    game_over: bool,
}

impl GameScene {
    fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        Ok(GameScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            background: Background::new(ctx)?,
            get_ready: Texture::new(ctx, "./resources/get-ready.png")?,
            instructions: Texture::new(ctx, "./resources/instructions.png")?,
            
            bird: Bird::new(ctx)?,

            flap_sound: Sound::new("./resources/flap.wav")?,
            ground_hit_sound: Sound::new("./resources/ground-hit.wav")?,
            pipe_hit_sound: Sound::new("./resources/pipe-hit.wav")?,
            score_sound: Sound::new("./resources/score.wav")?,
            ouch_sound: Sound::new("./resources/ouch.wav")?,
            drop_timer: 0,
            move_timer: 0,
            score: 0,
            score_text: Text::new("Score: 0", Font::default(), 16.0),

            is_mouse_down: true,
            instructions_visible: true,
            pipes: Vec::new(),
            game_over: false
        })
    }

    fn start_game(&mut self) {
        if self.instructions_visible {
            self.instructions_visible = false;
        }
        self.bird.reset();
        self.game_over = false;
        self.bird.allow_gravity = true;
        self.background.scroll = true;
    }

    fn check_for_collisions(&mut self) {
        if self.bird.collides_with(&self.background.get_collision_rect()) {
        // if check_collision(&self.background.get_collision_rect(), &self.bird.get_collision_rect()) {
            self.bird.allow_gravity = false;
            self.background.scroll = false;

            self.game_over = true;
        }
    }

    // fn collides(&mut self, move_x: i32, move_y: i32) -> bool {
    //     for (x, y) in self.block.segments() {
    //         let new_x = x + move_x;
    //         let new_y = y + move_y;

    //         if new_y < 0 {
    //             continue;
    //         }

    //         if new_x < 0
    //             || new_x > 9
    //             || new_y > 21
    //             || self.board[new_y as usize][new_x as usize].is_some()
    //         {
    //             return true;
    //         }
    //     }

    //     false
    // }

    // fn lock(&mut self) {
    //     let color = self.block.color();

    //     for (x, y) in self.block.segments() {
    //         if x >= 0 && x <= 9 && y >= 0 && y <= 21 {
    //             self.board[y as usize][x as usize] = Some(color);
    //         }
    //     }
    // }

    // fn check_for_clears(&mut self) -> bool {
    //     let mut cleared = false;

    //     'outer: for y in 0..22 {
    //         for x in 0..10 {
    //             if self.board[y][x].is_none() {
    //                 continue 'outer;
    //             }
    //         }

    //         cleared = true;

    //         self.score += 1;
    //         self.score_text
    //             .set_content(format!("Score: {}", self.score));

    //         for clear_y in (0..=y).rev() {
    //             if clear_y > 0 {
    //                 self.board[clear_y] = self.board[clear_y - 1];
    //             } else {
    //                 self.board[clear_y] = [None; 10];
    //             }
    //         }
    //     }

    //     cleared
    // }

    // fn check_for_game_over(&self) -> bool {
    //     self.board[0].iter().any(Option::is_some) || self.board[1].iter().any(Option::is_some)
    // }

    // fn board_blocks(&self) -> impl Iterator<Item = (i32, i32, Color)> + '_ {
    //     self.board.iter().enumerate().flat_map(|(y, row)| {
    //         row.iter()
    //             .enumerate()
    //             .filter(|(_, segment)| segment.is_some())
    //             .map(move |(x, segment)| (x as i32, y as i32, segment.unwrap()))
    //     })
    // }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.bird.update();

        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            if !self.is_mouse_down {
                if self.instructions_visible || self.game_over {
                    self.start_game();
                }
                self.bird.flap();
                self.is_mouse_down = true;
            }
        } else {
            self.is_mouse_down = false;
        }

        self.background.update();

        self.check_for_collisions();

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

        self.bird.draw(ctx);
    }
}
