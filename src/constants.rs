pub const SCALE: i32 = 20;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 640;

pub const FPS: f32 = 5.;

#[derive(Copy, Clone)]
pub enum Movement {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
