use rand::{self, Rng};
use tetra::audio::Sound;
use tetra::graphics::ScreenScaling;
use tetra::graphics::{self, Color, DrawParams, Font, Text, Texture, Rectangle, Vec2};
use tetra::graphics::animation::Animation;
use tetra::input::{self, Key};
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const SCREEN_WIDTH: i32 = 288;
const SCREEN_HEIGHT: i32 = 505;

fn main() -> tetra::Result {
    ContextBuilder::new("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable(true)
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
        graphics::set_scaling(ctx, ScreenScaling::None);
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
    ground_texture: Texture,

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
}

impl GameScene {
    fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        Ok(GameScene {
            sky_texture: Texture::new(ctx, "./resources/sky.png")?,
            ground_texture: Texture::new(ctx, "./resources/ground.png")?,
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

            rotation: 0.0
        })
    }

    fn flap(&mut self) {

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
        // self.drop_timer += 1;
        // self.move_timer += 1;

        // if self.drop_timer >= 30 {
        //     self.drop_timer = 0;
        //     self.move_queue.push(Move::Drop);
        // }

        if input::is_key_pressed(ctx, Key::Space)
        {
            // self.move_timer = 0;
            // self.move_queue.push(Move::Left);
        }

        // if input::is_key_pressed(ctx, Key::D)
        //     || (self.move_timer == 10 && input::is_key_down(ctx, Key::D))
        // {
        //     self.move_timer = 0;
        //     self.move_queue.push(Move::Right);
        // }

        // if input::is_key_pressed(ctx, Key::Q)
        //     || (self.move_timer == 10 && input::is_key_down(ctx, Key::Q))
        // {
        //     self.move_timer = 0;
        //     self.move_queue.push(Move::RotateCcw);
        // }

        // if input::is_key_pressed(ctx, Key::E)
        //     || (self.move_timer == 10 && input::is_key_down(ctx, Key::E))
        // {
        //     self.move_timer = 0;
        //     self.move_queue.push(Move::RotateCw);
        // }

        // if input::is_key_pressed(ctx, Key::S)
        //     || (self.move_timer == 10 && input::is_key_down(ctx, Key::S))
        // {
        //     self.move_timer = 0;
        //     self.drop_timer = 0;
        //     self.move_queue.push(Move::Drop);
        // }

        // if input::is_key_pressed(ctx, Key::Space) {
        //     self.drop_timer = 0;
        //     self.move_queue.push(Move::HardDrop);
        // }

        // let next_move = self.move_queue.pop();

        // match next_move {
        //     Some(Move::Left) => {
        //         if !self.collides(-1, 0) {
        //             self.block.x -= 1;
        //         }
        //     }
        //     Some(Move::Right) => {
        //         if !self.collides(1, 0) {
        //             self.block.x += 1;
        //         }
        //     }
        //     Some(Move::RotateCcw) => {
        //         self.block.rotate_ccw();

        //         let mut nudge = 0;

        //         if self.collides(0, 0) {
        //             nudge = if self.block.x > 5 { -1 } else { 1 }
        //         }

        //         if nudge != 0 && self.collides(nudge, 0) {
        //             self.block.rotate_cw();
        //         } else {
        //             self.block.x += nudge;
        //         }
        //     }
        //     Some(Move::RotateCw) => {
        //         self.block.rotate_cw();

        //         let mut nudge = 0;

        //         if self.collides(0, 0) {
        //             nudge = if self.block.x > 5 { -1 } else { 1 }
        //         }

        //         if nudge != 0 && self.collides(nudge, 0) {
        //             self.block.rotate_ccw();
        //         } else {
        //             self.block.x += nudge;
        //         }
        //     }
        //     Some(Move::Drop) => {
        //         if self.collides(0, 1) {
        //             self.soft_drop_sound.play_with(ctx, 0.5, 1.0)?;
        //             self.lock();

        //             if self.check_for_clears() {
        //                 self.line_clear_sound.play_with(ctx, 0.5, 1.0)?;
        //             }

        //             if self.check_for_game_over() {
        //                 self.game_over_sound.play_with(ctx, 0.2, 1.0)?;
        //                 return Ok(Transition::Pop);
        //             }

        //             self.block = Block::new();
        //         } else {
        //             self.block.y += 1;
        //         }
        //     }
        //     Some(Move::HardDrop) => {
        //         while !self.collides(0, 1) {
        //             self.block.y += 1;
        //         }

        //         self.hard_drop_sound.play_with(ctx, 0.5, 1.0)?;
        //         self.lock();

        //         if self.check_for_clears() {
        //             self.line_clear_sound.play_with(ctx, 0.5, 1.0)?;
        //         }

        //         if self.check_for_game_over() {
        //             self.game_over_sound.play_with(ctx, 0.2, 1.0)?;
        //             return Ok(Transition::Pop);
        //         }

        //         self.block = Block::new();
        //     }
        //     None => {}
        // }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.sky_texture, Vec2::new(0.0, 0.0));
        graphics::draw(ctx, &self.ground_texture, Vec2::new(0.0, 400.0));

        graphics::draw(
            ctx,
            &self.bird,
            DrawParams::new()
                .position(Vec2::new(100.0, 252.0))
                .origin(Vec2::new(0.0, 0.0))
                .rotation(self.rotation)
        );
    }
}
