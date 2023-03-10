#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::{Color32, Pos2, Rect, Stroke};
use tetris_gui::constants::{
    Movement, Rotation, FPS, GAME_WIDTH, HEIGHT, NEXT_PIECE_DISPLAY_WIDTH, SCALE,
};
use tetris_gui::game::{new_game, EndOfGame, Game, StepKind};

use std::time::{Duration, Instant};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(
            (GAME_WIDTH + NEXT_PIECE_DISPLAY_WIDTH) as f32,
            HEIGHT as f32,
        )),
        ..Default::default()
    };
    eframe::run_native(
        "Quattorix",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Copy, Clone)]
enum Command {
    None,
    Movement(Movement),
    Rotation(Rotation),
    DropDown,
}

struct MyApp {
    game: Game,
    time: Instant,
    fine_grained_time: Instant,
    current_move_command: Command,
    current_rotation_command: Command,
    is_paused: bool,
    game_over: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            game: new_game(),
            time: now,
            fine_grained_time: now,
            current_move_command: Command::None,
            current_rotation_command: Command::None,
            is_paused: false,
            game_over: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.game_over {
                ui.centered_and_justified(|ui| {
                    ui.heading(format!("Game Over!\nFinal Score: {}", self.game.score))
                });
                return;
            }

            let events = ui.input().events.clone();

            for event in &events {
                self.set_pause_or_unpause(event);

                self.set_move_command(event);

                self.set_rotation_command(event);
            }

            paint_rectangle(ui);

            if self.is_paused {
                ui.centered_and_justified(|ui| ui.heading("PAUSED"));
                return;
            } else {
                let score = self.game.score;
                ui.heading(format!("SCORE: {}", score));
            }

            self.paint_pieces(ui);
            self.paint_projection(ui);

            let time_now = Instant::now();
            let delta_t = time_now.duration_since(self.time);
            let fine_grained_delta_t = time_now.duration_since(self.fine_grained_time);

            if fine_grained_delta_t >= Duration::from_millis((10. * (1. / FPS)) as u64) {
                self.process_move_command(time_now);
            }

            if delta_t >= Duration::from_millis((2500. * (1. / FPS)) as u64) {
                let game_still_on = self.game.step(StepKind::GoDown);

                match game_still_on {
                    Ok(()) => self.game_over = false,
                    Err(EndOfGame) => self.game_over = true,
                }

                self.time = time_now;
                self.current_move_command = Command::None;
                self.current_rotation_command = Command::None;
            }
        });

        ctx.request_repaint();
    }
}

fn paint_rectangle(ui: &mut egui::Ui) {
    ui.painter().rect_stroke(
        Rect {
            min: Pos2 { x: 0., y: 0. },
            max: Pos2 {
                x: GAME_WIDTH as f32,
                y: HEIGHT as f32,
            },
        },
        0.,
        Stroke {
            width: 1.,
            color: Color32::GRAY,
        },
    );

    ui.painter().rect_stroke(
        Rect {
            min: Pos2 {
                x: GAME_WIDTH as f32,
                y: 0.,
            },
            max: Pos2 {
                x: (GAME_WIDTH + NEXT_PIECE_DISPLAY_WIDTH) as f32,
                y: HEIGHT as f32,
            },
        },
        0.,
        Stroke {
            width: 1.,
            color: Color32::GRAY,
        },
    );
}

impl MyApp {
    fn get_command(&self, pressed: &bool, key: &egui::Key) -> Command {
        if *pressed {
            match key {
                egui::Key::ArrowUp => Command::Movement(Movement::UP),
                egui::Key::ArrowDown => Command::Movement(Movement::DOWN),
                egui::Key::ArrowLeft => Command::Movement(Movement::LEFT),
                egui::Key::ArrowRight => Command::Movement(Movement::RIGHT),
                egui::Key::E => Command::Rotation(Rotation::CW),
                egui::Key::Q => Command::Rotation(Rotation::CCW),
                egui::Key::Z => Command::DropDown,
                _ => self.current_move_command,
            }
        } else {
            self.current_move_command
        }
    }

    fn paint_pieces(&self, ui: &mut egui::Ui) {
        for square in self.game.list_squares() {
            ui.painter().rect_filled(
                Rect {
                    min: Pos2 {
                        x: square.0[0] as f32 - SCALE as f32 / 2.,
                        y: square.0[1] as f32 - SCALE as f32 / 2.,
                    },
                    max: Pos2 {
                        x: square.0[0] as f32 + SCALE as f32 / 2.,
                        y: square.0[1] as f32 + SCALE as f32 / 2.,
                    },
                },
                SCALE as f32 / 5.,
                square.1,
            )
        }

        let next_piece = self.game.get_next_piece();

        for coord in next_piece.coords {
            ui.painter().rect_filled(
                Rect {
                    min: Pos2 {
                        x: coord[0] as f32 - SCALE as f32 / 2.,
                        y: coord[1] as f32 - SCALE as f32 / 2.,
                    },
                    max: Pos2 {
                        x: coord[0] as f32 + SCALE as f32 / 2.,
                        y: coord[1] as f32 + SCALE as f32 / 2.,
                    },
                },
                SCALE as f32 / 5.,
                next_piece.color,
            )
        }
    }

    fn paint_projection(&self, ui: &mut egui::Ui) {
        let mut phantom_piece = self.game.player_piece.clone();

        loop {
            let mut next_phantom_piece = phantom_piece.clone();
            next_phantom_piece.step_down();

            if next_phantom_piece.hits_bottom()
                || next_phantom_piece.intersect(
                    &self
                        .game
                        .frozen_squares
                        .iter()
                        .map(|colored_point| colored_point.0)
                        .collect(),
                )
            {
                break;
            } else {
                phantom_piece = next_phantom_piece;
            }
        }

        for square in phantom_piece.coords {
            ui.painter().rect_stroke(
                Rect {
                    min: Pos2 {
                        x: square[0] as f32 - SCALE as f32 / 2.,
                        y: square[1] as f32 - SCALE as f32 / 2.,
                    },
                    max: Pos2 {
                        x: square[0] as f32 + SCALE as f32 / 2.,
                        y: square[1] as f32 + SCALE as f32 / 2.,
                    },
                },
                SCALE as f32 / 5.,
                Stroke {
                    width: 1.,
                    color: phantom_piece.color,
                },
            )
        }
    }

    fn set_pause_or_unpause(&mut self, event: &egui::Event) {
        self.is_paused = match event {
            egui::Event::Key {
                key: egui::Key::Space,
                pressed,
                modifiers: _,
            } => {
                if *pressed {
                    !self.is_paused
                } else {
                    self.is_paused
                }
            }
            _ => self.is_paused,
        }
    }

    fn set_move_command(&mut self, event: &egui::Event) {
        self.current_move_command = match event {
            egui::Event::Key {
                key,
                pressed,
                modifiers: _,
            } => match self.get_command(pressed, key) {
                Command::Movement(x) => Command::Movement(x),
                Command::DropDown => Command::DropDown,
                _ => self.current_move_command,
            },
            _ => self.current_move_command,
        };
    }

    fn set_rotation_command(&mut self, event: &egui::Event) {
        self.current_rotation_command = match event {
            egui::Event::Key {
                key,
                pressed,
                modifiers: _,
            } => {
                if let Command::Rotation(x) = self.get_command(pressed, key) {
                    Command::Rotation(x)
                } else {
                    self.current_rotation_command
                }
            }
            _ => self.current_rotation_command,
        }
    }

    fn process_move_command(&mut self, time_now: Instant) {
        match self.current_move_command {
            Command::Movement(movement) => self.game.step(StepKind::Move(Some(movement))).unwrap(),
            Command::DropDown => self.game.step(StepKind::HardDrop).unwrap(),
            _ => (),
        }
        if let Command::Rotation(rotation) = self.current_rotation_command {
            self.game.step(StepKind::Rotate(rotation)).unwrap()
        }
        self.fine_grained_time = time_now;
        self.current_move_command = Command::None;
        self.current_rotation_command = Command::None;
    }
}
