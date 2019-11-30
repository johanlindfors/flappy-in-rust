use tetra::graphics::Rectangle;

pub trait PhysicsBody {
    fn get_collision_rect(&mut self) -> Rectangle;
    fn collides_with(&mut self, obj: &Rectangle) -> bool;
}

pub fn check_collision(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.x < rect2.x + rect2.width
        && rect1.x + rect1.width > rect2.x
        && rect1.y < rect2.y + rect2.height
        && rect1.y + rect1.height > rect2.y
}
