pub const SCALE: i32 = 30;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;

pub const FPS: f32 = 4.;

#[derive(Copy, Clone)]
pub enum Movement {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
