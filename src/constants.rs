pub const SCALE: i32 = 40;

pub const GAME_WIDTH: i32 = 400;
pub const NEXT_PIECE_DISPLAY_WIDTH: i32 = 240;
pub const HEIGHT: i32 = 900;

pub const FPS: f32 = 3.;

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
