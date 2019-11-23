use tetra::graphics::{self, Animation, Texture, Rectangle, DrawParams, Vec2};
use tetra::{Context};

use crate::systems::physics::{PhysicsBody, check_collision};
use crate::{SCREEN_HEIGHT, GRAVITY};

pub struct Bird {
    animation: Animation,
    rotation: f32,
    velocity: Vec2,
    flap_counter: i32,
    flap_delta: f64,
    
    pub position: Vec2,
    pub allow_gravity: bool,
    pub alive: bool,
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
    pub fn new(ctx: &mut Context) -> tetra::Result<Bird> {
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

    pub fn flap(&mut self) {
        if self.alive {
            self.velocity.y = -6.5;
            self.flap_counter = 6;
            self.tween_rotation();
        }
    }

    pub fn kill(&mut self) {
        if self.alive && self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
        }
        self.alive = false;
    }

    pub fn reset(&mut self) {
        self.velocity = Vec2::new(0.0, 0.0);
        self.position = Vec2::new(100.0, SCREEN_HEIGHT as f32 / 2.0);
        self.rotation = 0.0;
        self.flap_delta = 0.0;
        self.alive = true;
    }

    pub fn tween_rotation(&mut self) {
        let distance = (-1.0 - self.rotation) as f64;
        self.flap_delta = distance.abs() / self.flap_counter as f64;
    }

    pub fn update(&mut self) {
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
            } if self.rotation < 1.5 {
                self.rotation += 0.05;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context ) {
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
