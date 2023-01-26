use egui::Color32;

use crate::{
    constants::{Movement, Rotation, HEIGHT, SCALE, WIDTH},
    pieces::{get_piece_from_above, Piece, PieceShape},
};

#[derive(Clone, Copy)]
pub struct ColoredPoint(pub [i32; 2], pub Color32);

pub struct Game {
    frozen_squares: Vec<ColoredPoint>,
    player_piece: Piece,
    pub score: i32,
}

pub fn new_game() -> Game {
    let random_shape: PieceShape = rand::random();

    Game {
        frozen_squares: Vec::new(),
        player_piece: get_piece_from_above(random_shape),
        score: 0,
    }
}

enum Outcome {
    DoNothing,
    Stick,
    Free,
}

pub enum StepKind<T, R> {
    Move(T),
    Rotate(R),
    GoDown,
    HardDrop,
}

enum SoftDropEnd {
    Yes,
    No,
}

impl Game {
    pub fn step(&mut self, step_kind: StepKind<Option<Movement>, Rotation>) -> Result<(), ()> {
        match step_kind {
            StepKind::GoDown => self.force_piece_down_or_stick()?,
            StepKind::Move(movement) => {
                let is_soft_drop_end = self.move_piece(movement);
                match is_soft_drop_end {
                    SoftDropEnd::Yes => {
                        self.score += self.player_piece.coords.iter().count() as i32
                    }
                    _ => (),
                }
            }
            StepKind::Rotate(rotation) => self.rotate_piece(rotation),
            StepKind::HardDrop => {
                self.score += 2 * self.player_piece.coords.iter().count() as i32;
                self.drop_down()
            }
        }

        let full_lines_heights = self.get_full_lines_heights();

        if !full_lines_heights.is_empty() {
            let n = full_lines_heights.iter().count() as i32;
            self.score += n.pow(2) * 100;
            self.erase_lines(full_lines_heights);
        };

        Ok(())
    }

    fn rotate_piece(&mut self, rotation: Rotation) {
        let mut phantom_piece = self.player_piece.clone();

        let mut outcome: Outcome = Outcome::Free;

        match rotation {
            Rotation::CCW => phantom_piece.rotate_ccw(),
            Rotation::CW => phantom_piece.rotate_cw(),
        };

        if phantom_piece.intersect(
            &self
                .frozen_squares
                .iter()
                .map(|colored_point| colored_point.0)
                .collect(),
        ) {
            outcome = Outcome::Stick;
        }

        if phantom_piece.hits_sides() {
            outcome = Outcome::DoNothing;
        }

        if phantom_piece.hits_bottom() {
            outcome = Outcome::DoNothing
        }

        match outcome {
            Outcome::Free => self.player_piece = phantom_piece,
            _ => (),
        }
    }

    fn move_piece(&mut self, movement: Option<Movement>) -> SoftDropEnd {
        let mut phantom_piece = self.player_piece.clone();

        let mut outcome: Outcome = Outcome::Free;

        phantom_piece.make_move(movement);

        if phantom_piece.hits_bottom() {
            outcome = Outcome::Stick
        }

        if phantom_piece.intersect(
            &self
                .frozen_squares
                .iter()
                .map(|colored_point| colored_point.0)
                .collect(),
        ) {
            match movement {
                Some(movement) => match movement {
                    Movement::DOWN => outcome = Outcome::Stick,
                    _ => outcome = Outcome::DoNothing,
                },
                _ => outcome = Outcome::DoNothing,
            }
        }

        if phantom_piece.hits_sides() {
            outcome = Outcome::DoNothing;
        }

        match outcome {
            Outcome::Free => {
                self.player_piece = phantom_piece;
                return SoftDropEnd::No;
            }
            Outcome::Stick => {
                let new_piece: PieceShape = rand::random();
                self.frozen_squares.extend(
                    self.player_piece
                        .coords
                        .iter()
                        .map(|coord| ColoredPoint(*coord, phantom_piece.color)),
                );

                self.player_piece = get_piece_from_above(new_piece);
                return SoftDropEnd::Yes;
            }
            _ => return SoftDropEnd::No,
        }
    }

    fn force_piece_down_or_stick(&mut self) -> Result<(), ()> {
        let mut phantom_piece = self.player_piece.clone();
        let mut outcome: Outcome = Outcome::Free;

        phantom_piece.step_down();

        if phantom_piece.hits_bottom() {
            outcome = Outcome::Stick
        }

        if phantom_piece.intersect(
            &self
                .frozen_squares
                .iter()
                .map(|colored_point| colored_point.0)
                .collect(),
        ) {
            outcome = Outcome::Stick;
        }

        match outcome {
            Outcome::Stick => {
                let new_piece: PieceShape = rand::random();
                self.frozen_squares.extend(
                    self.player_piece
                        .coords
                        .iter()
                        .map(|coord| ColoredPoint(*coord, phantom_piece.color)),
                );

                let old_piece = self.player_piece.clone();
                self.player_piece = get_piece_from_above(new_piece);

                if self.player_piece.intersect(&old_piece.coords) {
                    return Err(());
                }
            }
            Outcome::Free => {
                self.player_piece = phantom_piece;
            }
            Outcome::DoNothing => {
                self.player_piece = phantom_piece;
            }
        }

        return Ok(());
    }

    fn drop_down(&mut self) {
        let mut phantom_piece = self.player_piece.clone();

        loop {
            let mut next_phantom_piece = phantom_piece.clone();
            next_phantom_piece.step_down();

            if next_phantom_piece.hits_bottom()
                || next_phantom_piece.intersect(
                    &self
                        .frozen_squares
                        .iter()
                        .map(|colored_point| colored_point.0)
                        .collect(),
                )
            {
                let new_piece: PieceShape = rand::random();
                self.frozen_squares.extend(
                    phantom_piece
                        .coords
                        .iter()
                        .map(|coord| ColoredPoint(*coord, phantom_piece.color)),
                );

                self.player_piece = get_piece_from_above(new_piece);

                break;
            } else {
                phantom_piece = next_phantom_piece;
            }
        }
    }

    pub fn list_squares(&self) -> Vec<ColoredPoint> {
        let mut colored_points = self.frozen_squares.clone();

        for coord in self.player_piece.coords.iter() {
            colored_points.push(ColoredPoint(*coord, self.player_piece.color))
        }

        colored_points
    }

    fn get_full_lines_heights(&self) -> Vec<i32> {
        let mut full_lines_heights = Vec::new();

        for i in 0..HEIGHT / SCALE {
            let frozen_squares = self.frozen_squares.iter();

            let line_i = frozen_squares.filter(|tup| tup.0[1] == (SCALE * i));

            if line_i.count() == ((WIDTH / SCALE) - 1) as usize {
                full_lines_heights.push(SCALE * i);
            }
        }

        full_lines_heights
    }

    fn erase_lines(&mut self, lines: Vec<i32>) {
        let mut new_squares = Vec::new();

        for square in self.frozen_squares.iter() {
            if !lines.contains(&square.0[1]) {
                let lines_below = lines.iter().filter(|x| **x >= square.0[1]).count();

                new_squares.push(ColoredPoint(
                    [square.0[0], square.0[1] + SCALE * lines_below as i32],
                    square.1,
                ))
            }
        }

        self.frozen_squares = new_squares;
    }
}
