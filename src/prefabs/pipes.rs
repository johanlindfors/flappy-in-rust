use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

use crate::systems::physics::{check_collision, PhysicsBody};
use crate::SCROLL_SPEED;

pub struct Pipe {
    position: Vec2<f32>,
    source_rect: Rectangle,
}

impl Pipe {
    fn new(position: Vec2<f32>, source_rect: Rectangle) -> Self {
        Pipe {
            position,
            source_rect,
        }
    }

    fn draw(&mut self, ctx: &mut Context, position: Vec2<f32>, texture: &Texture) {
        texture.draw_region(
            ctx,
            self.source_rect,
            Vec2::new(self.position.x + position.x, self.position.y + position.y),
        );
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

pub struct PipeGroup {
    top_pipe: Pipe,
    bottom_pipe: Pipe,

    pub position: Vec2<f32>,
    pub alive: bool,
    pub enabled: bool,
    pub has_scored: bool,
}

impl PipeGroup {
    pub fn new() -> tetra::Result<PipeGroup> {
        Ok(PipeGroup {
            position: Vec2::new(0.0, 0.0),
            top_pipe: Pipe::new(Vec2::new(0.0, 0.0), Rectangle::new(0.0, 0.0, 54.0, 320.0)),
            bottom_pipe: Pipe::new(
                Vec2::new(0.0, 440.0),
                Rectangle::new(54.0, 0.0, 54.0, 320.0),
            ),
            alive: false,
            enabled: false,
            has_scored: false,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        if self.alive && self.enabled {
            self.position.x -= SCROLL_SPEED;
        }
        if self.position.x < -54.0 {
            self.alive = false;
            self.enabled = false;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, texture: &Texture) {
        self.top_pipe.draw(ctx, self.position, texture);
        self.bottom_pipe.draw(ctx, self.position, texture);
    }

    pub fn reset(&mut self, x: f32, y: f32) {
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
        let relative_rect = Rectangle::new(
            obj.x - self.position.x - 27.0,
            obj.y - self.position.y - 12.0,
            obj.width,
            obj.height,
        );
        self.top_pipe.collides_with(&relative_rect)
            || self.bottom_pipe.collides_with(&relative_rect)
    }
}

pub struct PipeGenerator {
    counter: i32,
    enabled: bool,
}

impl PipeGenerator {
    pub fn new() -> tetra::Result<PipeGenerator> {
        Ok(PipeGenerator {
            counter: 0,
            enabled: false,
        })
    }

    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn should_spawn_pipe(&mut self) -> bool {
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
