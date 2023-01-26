#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::{Color32, Pos2, Rect, Stroke};
use tetris::constants::{Movement, Rotation, FPS, HEIGHT, SCALE, WIDTH};
use tetris::game::{new_game, Game, StepKind};

use std::time::{Duration, Instant};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    eframe::run_native(
        "Tetris",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Copy, Clone)]
enum Command<T, R> {
    NoCommand,
    Movement(T),
    Rotation(R),
    DropDown,
}

struct MyApp {
    game: Game,
    time: Instant,
    fine_grained_time: Instant,
    current_move_command: Command<Movement, Rotation>,
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
            current_move_command: Command::NoCommand,
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
                self.is_paused = match event {
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers: _,
                    } => match key {
                        egui::Key::Space => {
                            if *pressed {
                                !self.is_paused
                            } else {
                                self.is_paused
                            }
                        }
                        _ => self.is_paused,
                    },
                    _ => self.is_paused,
                };

                self.current_move_command = match event {
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers: _,
                    } => self.get_command(pressed, key),
                    _ => self.current_move_command,
                }
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

            let time_now = Instant::now();
            let delta_t = time_now.duration_since(self.time);
            let fine_grained_delta_t = time_now.duration_since(self.fine_grained_time);

            if fine_grained_delta_t >= Duration::from_millis((77. * (1. / FPS)) as u64) {
                match self.current_move_command {
                    Command::Movement(movement) => {
                        self.game.step(StepKind::Move(Some(movement))).unwrap()
                    }
                    Command::Rotation(rotation) => {
                        self.game.step(StepKind::Rotate(rotation)).unwrap()
                    }
                    Command::DropDown => self.game.step(StepKind::HardDrop).unwrap(),
                    _ => (),
                }
                self.fine_grained_time = time_now;
                self.current_move_command = Command::NoCommand;
            }

            if delta_t >= Duration::from_millis((1000. * (1. / FPS)) as u64) {
                let game_still_on = self.game.step(StepKind::GoDown);

                match game_still_on {
                    Ok(()) => self.game_over = false,
                    Err(()) => self.game_over = true,
                }

                self.time = time_now;
                self.current_move_command = Command::NoCommand;
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
                x: WIDTH as f32,
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
    fn get_command(&self, pressed: &bool, key: &egui::Key) -> Command<Movement, Rotation> {
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

    fn paint_pieces(&mut self, ui: &mut egui::Ui) {
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
    }
}
