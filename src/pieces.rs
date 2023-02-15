use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};
use std::collections::HashSet;

use crate::constants::{Movement, HEIGHT, SCALE, WIDTH};
use egui::Color32;

pub enum PieceShape {
    L,
    I,
    Z,
    InvertedZ,
    InvertedL,
    Square,
    T,
}

impl Distribution<PieceShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceShape {
        match rng.gen_range(0..=6) {
            0 => PieceShape::L,
            1 => PieceShape::I,
            2 => PieceShape::Z,
            3 => PieceShape::Square,
            4 => PieceShape::T,
            5 => PieceShape::InvertedZ,
            6 => PieceShape::InvertedL,
            _ => panic!(),
        }
    }
}

impl PieceShape {
    fn get_piece_shape_from_int(i: usize) -> Option<PieceShape> {
        match i {
            0 => Some(PieceShape::L),
            1 => Some(PieceShape::I),
            2 => Some(PieceShape::Z),
            3 => Some(PieceShape::Square),
            4 => Some(PieceShape::T),
            5 => Some(PieceShape::InvertedZ),
            6 => Some(PieceShape::InvertedL),
            _ => None,
        }
    }

    pub fn generate_fair_collection(n: usize) -> Vec<PieceShape> {
        let mut possibilities = (0..=6).cycle().take(n * 7).collect::<Vec<usize>>();
        possibilities.shuffle(&mut thread_rng());

        possibilities
            .into_iter()
            .map(|x| PieceShape::get_piece_shape_from_int(x).unwrap())
            .collect()
    }
}

pub fn get_piece_from_above(piece_shape: PieceShape) -> Piece {
    match piece_shape {
        PieceShape::L => Piece {
            coords: vec![
                [WIDTH / 2, 0],
                [WIDTH / 2, SCALE],
                [WIDTH / 2, 2 * SCALE],
                [WIDTH / 2 + SCALE, 2 * SCALE],
            ],
            center: [WIDTH / 2, SCALE],
            color: Color32::from_rgb(255, 165, 0),
        },
        PieceShape::InvertedL => Piece {
            coords: vec![
                [WIDTH / 2, 0],
                [WIDTH / 2, SCALE],
                [WIDTH / 2, 2 * SCALE],
                [WIDTH / 2 - SCALE, 2 * SCALE],
            ],
            center: [WIDTH / 2, SCALE],
            color: Color32::WHITE,
        },
        PieceShape::I => Piece {
            coords: vec![
                [WIDTH / 2, 0],
                [WIDTH / 2, SCALE],
                [WIDTH / 2, 2 * SCALE],
                [WIDTH / 2, 3 * SCALE],
            ],
            center: [WIDTH / 2, 2 * SCALE],
            color: Color32::from_rgb(173, 216, 230),
        },
        PieceShape::Z => Piece {
            coords: vec![
                [WIDTH / 2, 0],
                [WIDTH / 2, SCALE],
                [WIDTH / 2 + SCALE, SCALE],
                [WIDTH / 2 + SCALE, 2 * SCALE],
            ],
            center: [WIDTH / 2, SCALE],
            color: Color32::GREEN,
        },
        PieceShape::Square => Piece {
            coords: vec![
                [WIDTH / 2, 0],
                [WIDTH / 2, SCALE],
                [WIDTH / 2 + SCALE, SCALE],
                [WIDTH / 2 + SCALE, 0],
            ],
            center: [WIDTH / 2 + SCALE / 2, SCALE / 2],
            color: Color32::from_rgb(255, 255, 102),
        },
        PieceShape::T => Piece {
            coords: vec![
                [WIDTH / 2 - SCALE, 0],
                [WIDTH / 2, 0],
                [WIDTH / 2 + SCALE, 0],
                [WIDTH / 2, SCALE],
            ],
            center: [WIDTH / 2, 0],
            color: Color32::from_rgb(255, 20, 147),
        },
        PieceShape::InvertedZ => Piece {
            coords: vec![
                [WIDTH / 2 + SCALE, 0],
                [WIDTH / 2 + SCALE, SCALE],
                [WIDTH / 2, SCALE],
                [WIDTH / 2, 2 * SCALE],
            ],
            center: [WIDTH / 2, SCALE],
            color: Color32::RED,
        },
    }
}

#[derive(Clone)]
pub struct Piece {
    pub coords: Vec<[i32; 2]>,
    pub center: [i32; 2],
    pub color: Color32,
}

impl Piece {
    pub fn rotate_ccw(&mut self) {
        let translated_coords: Vec<[i32; 2]> = self
            .coords
            .iter()
            .map(|tup| [tup[0] - self.center[0], tup[1] - self.center[1]])
            .collect();

        self.coords = translated_coords
            .iter()
            .map(|tup| [-tup[1] + self.center[0], tup[0] + self.center[1]])
            .collect();
    }

    pub fn rotate_cw(&mut self) {
        let translated_coords: Vec<[i32; 2]> = self
            .coords
            .iter()
            .map(|tup| [tup[0] - self.center[0], tup[1] - self.center[1]])
            .collect();

        self.coords = translated_coords
            .iter()
            .map(|tup| [tup[1] + self.center[0], -tup[0] + self.center[1]])
            .collect();
    }

    pub fn step_down(&mut self) {
        self.coords = self
            .coords
            .iter()
            .map(|tup| [tup[0], tup[1] + SCALE])
            .collect();
        self.center = [self.center[0], self.center[1] + SCALE]
    }

    pub fn make_move(&mut self, movement: Option<Movement>) {
        self.coords = match movement {
            Some(x) => match x {
                Movement::UP => self.coords.clone(),
                Movement::DOWN => self
                    .coords
                    .iter()
                    .map(|tup| [tup[0], tup[1] + SCALE])
                    .collect(),
                Movement::RIGHT => self
                    .coords
                    .iter()
                    .map(|tup| [tup[0] + SCALE, tup[1]])
                    .collect(),
                Movement::LEFT => self
                    .coords
                    .iter()
                    .map(|tup| [tup[0] - SCALE, tup[1]])
                    .collect(),
            },
            None => self.coords.clone(),
        };

        self.center = match movement {
            Some(x) => match x {
                Movement::UP => self.center,
                Movement::DOWN => [self.center[0], self.center[1] + SCALE],
                Movement::RIGHT => [self.center[0] + SCALE, self.center[1]],
                Movement::LEFT => [self.center[0] - SCALE, self.center[1]],
            },
            None => self.center,
        };
    }

    pub fn intersect(&self, points: &Vec<[i32; 2]>) -> bool {
        let my_pieces: HashSet<&[i32; 2]> = HashSet::from_iter(self.coords.iter());
        let points_set: HashSet<&[i32; 2]> = HashSet::from_iter(points.iter());

        my_pieces.intersection(&points_set).count() > 0
    }

    pub fn hits_bottom(&self) -> bool {
        for coord in self.coords.clone() {
            if coord[1] >= HEIGHT {
                return true;
            }
        }

        return false;
    }
    pub fn hits_sides(&self) -> bool {
        for coord in self.coords.clone() {
            if coord[0] >= WIDTH || coord[0] <= 0 {
                return true;
            }
        }

        return false;
    }
}
