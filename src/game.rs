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

impl Game {
    pub fn step(&mut self, movement: Option<Movement>) {
        let mut phantom_piece = self.player_piece.clone();

        phantom_piece.make_move(movement);
        phantom_piece.step_down();

        let mut to_get_stuck: bool = false;

        if phantom_piece.hits_border() {
            to_get_stuck = true
        } else {
            for piece in self.frozen_pieces.iter() {
                if phantom_piece.intersect(&piece) {
                    to_get_stuck = true;
                }
            }
        }

        if to_get_stuck {
            let new_piece: PieceShape = rand::random();
            self.frozen_pieces.push(self.player_piece.clone());
            self.player_piece = get_piece_from_above(new_piece);
        } else {
            self.player_piece.make_move(movement);
            self.player_piece.step_down();
        }
    }

    pub fn list_pieces(&self) -> Vec<Piece> {
        let mut pieces = self.frozen_pieces.clone();

        pieces.push(self.player_piece.clone());

        pieces
    }
}
