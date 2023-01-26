use crate::{
    constants::{Movement, Rotation, HEIGHT, SCALE, WIDTH},
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

                if phantom_piece.hits_bottom() {
                    outcome = Outcome::Stick
                }

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

    fn get_frozen_squares(&self) -> Vec<[i32; 2]> {
        let mut squares = Vec::new();

        for frozen_square in self.frozen_pieces.iter() {
            for coord in frozen_square.coords.iter() {
                squares.push(*coord);
            }
        }

        squares
    }

    fn get_full_lines_heights(&self) -> Vec<i32> {
        let mut full_lines_heights = Vec::new();

        for i in 0..HEIGHT / SCALE {
            let frozen_squares = self.get_frozen_squares();

            let line_i = frozen_squares.iter().filter(|tup| tup[1] == (SCALE * i));
            if line_i.count() == (WIDTH / SCALE) as usize {
                full_lines_heights.push(SCALE * i);
            }
        }

        full_lines_heights
    }
}
