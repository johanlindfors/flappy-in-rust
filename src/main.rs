use rand::{self, Rng};
use tetra::audio::Sound;
use tetra::graphics::ScreenScaling;
use tetra::graphics::{self, Color, DrawParams, Font, Text, Texture, Rectangle, Vec2};
use tetra::graphics::animation::Animation;
use tetra::input::{self, Key};
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

// === Tween manager ===

trait Tweenable {
    fn is_complete(&mut self) -> bool;
    fn update(&mut self, delta: f64);
}

struct TweenManager {

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
        })
    }

    fn update(&mut self) {
        self.ground_rect.x += 4.0 ;
        self.forest_rect.x += 3.0 ;
        self.cityscape_rect.x += 2.0 ;
        self.cloud_rect.x += 1.0 ;
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

// === Title Scene ===

struct TitleScene {
    title_text: Text,
    help_text: Text,
}

impl TitleScene {
    fn new(ctx: &mut Context) -> tetra::Result<TitleScene> {
        // // Setting a Sound to repeat without holding on to the SoundInstance
        // // is usually a bad practice, as it means you can never stop playback.
        // // In our case though, we want it to repeat forever, so it's fine!
        // Sound::new("./examples/resources/bgm.wav")?.repeat(ctx)?;

        Ok(TitleScene {
            title_text: Text::new("Flappy Bird", Font::default(), 36.0),
            help_text: Text::new("An extremely legally distinct puzzle game\n\nControls:\nA and D to move\nQ and E to rotate\nS to drop one row\nSpace to hard drop\n\nPress Space to start.", Font::default(), 16.0),
        })
    }
}

impl Scene for TitleScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_pressed(ctx, Key::Space) {
            Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        graphics::draw(ctx, &self.title_text, Vec2::new(16.0, 16.0));
        graphics::draw(ctx, &self.help_text, Vec2::new(16.0, 56.0));
    }
}

// === Game Scene ===

struct GameScene {
    sky_texture: Texture,
    background: Background,

    bird: Animation,
    
    flap_sound: Sound,
    ground_hit_sound: Sound,
    pipe_hit_sound: Sound,
    score_sound: Sound,
    ouch_sound: Sound,

    drop_timer: i32,
    move_timer: i32,
    score: i32,
    score_text: Text,

    rotation: f32,
    position: Vec2,
    velocity: Vec2,
    flap_counter: i32,
    flap_delta: f64,

}

impl GameScene {
    fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        Ok(GameScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            background: Background::new(ctx)?,
            
            bird: Animation::new(
                Texture::new(ctx, "./resources/bird.png")?,
                Rectangle::row(0.0, 0.0, 34.0, 24.0).take(3).collect(),
                5,
            ),

            flap_sound: Sound::new("./resources/flap.wav")?,
            ground_hit_sound: Sound::new("./resources/ground-hit.wav")?,
            pipe_hit_sound: Sound::new("./resources/pipe-hit.wav")?,
            score_sound: Sound::new("./resources/score.wav")?,
            ouch_sound: Sound::new("./resources/ouch.wav")?,
            drop_timer: 0,
            move_timer: 0,
            score: 0,
            score_text: Text::new("Score: 0", Font::default(), 16.0),

            rotation: 0.0,
            position: Vec2::new(100.0, 252.0),
            velocity: Vec2::new(0.0, 0.0),
            flap_counter: 0,
            flap_delta: 0.0,
        })
    }

    fn flap(&mut self) {
        self.velocity.y = -7.5;
        self.flap_counter = 6;
        self.tween_rotation();
    }

    fn tween_rotation(&mut self) {
        let distance = (-1.0 - self.rotation) as f64;
        self.flap_delta = distance.abs() / self.flap_counter as f64;
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
        self.bird.tick();

        if input::is_key_pressed(ctx, Key::Space) {
            self.flap();
        }

        self.velocity.y = self.velocity.y + GRAVITY / 30.0;
        self.position.y = self.position.y + self.velocity.y;
        
        if self.flap_counter > 0 {
            self.rotation -= self.flap_delta as f32;
            self.flap_counter -= 1; 
        } if self.rotation < 1.3 {
            self.rotation += 0.05;
        }

        self.background.update();

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.sky_texture, Vec2::new(0.0, 0.0));

        self.background.draw(ctx);

        graphics::draw(
            ctx,
            &self.bird,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(17.0, 12.0))
                .rotation(self.rotation)
        );
    }
}
