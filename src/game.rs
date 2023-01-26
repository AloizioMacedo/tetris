use crate::{
    constants::Movement,
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
    SlideDown,
    StickImmediately,
    StickAfterMove,
    Free,
}

impl Game {
    pub fn step(&mut self, movement: Option<Movement>) {
        let mut phantom_piece = self.player_piece.clone();

        let mut outcome: Outcome = Outcome::Free;
        phantom_piece.make_move(movement);

        for piece in self.frozen_pieces.iter() {
            if phantom_piece.intersect(&piece) {
                outcome = Outcome::SlideDown;
            }
        }

        match outcome {
            Outcome::SlideDown => phantom_piece = self.player_piece.clone(),
            _ => (),
        }

        phantom_piece.step_down();

        if phantom_piece.hits_bottom() {
            outcome = Outcome::StickImmediately
        } else if phantom_piece.hits_sides() {
            outcome = Outcome::SlideDown
        } else {
            for piece in self.frozen_pieces.iter() {
                if phantom_piece.intersect(&piece) {
                    outcome = match outcome {
                        Outcome::Free => Outcome::StickAfterMove,
                        Outcome::SlideDown => Outcome::StickImmediately,
                        rest => rest,
                    };
                }
            }
        }

        match outcome {
            Outcome::StickImmediately => {
                let new_piece: PieceShape = rand::random();
                self.frozen_pieces.push(self.player_piece.clone());
                self.player_piece = get_piece_from_above(new_piece);
            }
            Outcome::StickAfterMove => {
                let new_piece: PieceShape = rand::random();
                let mut to_freeze = self.player_piece.clone();
                to_freeze.make_move(movement);
                self.frozen_pieces.push(to_freeze);
                self.player_piece = get_piece_from_above(new_piece);
            }
            Outcome::Free => {
                self.player_piece.make_move(movement);
                self.player_piece.step_down();
            }
            Outcome::SlideDown => {
                self.player_piece.step_down();
            }
        }
    }

    pub fn list_pieces(&self) -> Vec<Piece> {
        let mut pieces = self.frozen_pieces.clone();

        pieces.push(self.player_piece.clone());

        pieces
    }
}
