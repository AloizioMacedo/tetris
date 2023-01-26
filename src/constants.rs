pub const SCALE: i32 = 40;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;

pub const FPS: f32 = 5.;

#[derive(Copy, Clone)]
pub enum Movement {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Copy, Clone)]
pub enum Rotation {
    CW,
    CCW,
}
