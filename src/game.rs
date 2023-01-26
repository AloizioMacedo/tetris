use crate::{
    constants::{Movement, Rotation},
    pieces::{get_piece_from_above, Piece, PieceShape},
};

pub struct Game {
    frozen_pieces: Vec<Piece>,
    player_piece: Piece,
}

pub fn new_game() -> Game {
    let random_shape: PieceShape = rand::random();

    Game {
        frozen_pieces: Vec::new(),
        player_piece: get_piece_from_above(random_shape),
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
}

impl Game {
    pub fn step(&mut self, step_kind: StepKind<Option<Movement>, Rotation>) {
        let mut phantom_piece = self.player_piece.clone();

        let mut outcome: Outcome = Outcome::Free;

        match step_kind {
            StepKind::GoDown => {
                phantom_piece.step_down();

                if phantom_piece.hits_bottom() {
                    outcome = Outcome::Stick
                }

                for piece in self.frozen_pieces.iter() {
                    if phantom_piece.intersect(&piece) {
                        outcome = Outcome::Stick;
                    }
                }

                match outcome {
                    Outcome::Stick => {
                        let new_piece: PieceShape = rand::random();
                        self.frozen_pieces.push(self.player_piece.clone());
                        self.player_piece = get_piece_from_above(new_piece);
                    }
                    Outcome::Free => {
                        self.player_piece = phantom_piece;
                    }
                    Outcome::DoNothing => {
                        self.player_piece = phantom_piece;
                    }
                }
            }
            StepKind::Move(movement) => {
                phantom_piece.make_move(movement);

                for piece in self.frozen_pieces.iter() {
                    if phantom_piece.intersect(&piece) {
                        outcome = Outcome::DoNothing;
                    }
                }

                if phantom_piece.hits_sides() {
                    outcome = Outcome::DoNothing;
                }

                match outcome {
                    Outcome::Free => self.player_piece = phantom_piece,
                    _ => (),
                }
            }
            StepKind::Rotate(rotation) => {
                match rotation {
                    Rotation::CCW => phantom_piece.rotate_ccw(),
                    Rotation::CW => phantom_piece.rotate_cw(),
                };

                for piece in self.frozen_pieces.iter() {
                    if phantom_piece.intersect(&piece) {
                        outcome = Outcome::DoNothing
                    }
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
        }
    }

    pub fn list_pieces(&self) -> Vec<Piece> {
        let mut pieces = self.frozen_pieces.clone();

        pieces.push(self.player_piece.clone());

        pieces
    }
}
